//! Lexical boundaries of SQL text under one backend's rules: where quoted
//! text, comments, parameters and words begin and end. It is not a grammar.
//!
//! The builder finds the markers of SQL fragments with it, the engine reads
//! raw statements with it, and TideORM inspects the SQL it runs with it.

/// Whose lexical rules a scan follows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dialect {
    /// SQL that may be meant for any backend: PostgreSQL's rules, plus
    /// SQLite's `[..]` identifiers, and an unclosed dollar quote read as a
    /// plain `$`.
    Any,
    /// MySQL and MariaDB: `#` comments, `--` only before whitespace,
    /// backslash escapes in strings, block comments that do not nest.
    MySql,
    /// PostgreSQL: nested block comments, dollar quotes, `$n` parameters,
    /// backslash escapes only in `E'..'` strings.
    Postgres,
    /// SQLite: `[..]` identifiers, block comments that do not nest, named
    /// `$`/`:`/`@`/`#` parameters, no backslash escapes.
    Sqlite,
}

/// What a [`Token`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// An unquoted word: a keyword or a name.
    Word,
    /// A quoted identifier (and, on MySQL, a double-quoted string).
    Identifier,
    /// A single-quoted or dollar-quoted string.
    Literal,
    /// A line or block comment.
    Comment,
    /// A run of whitespace.
    Space,
    /// Any other single character.
    Symbol,
    /// A `$n` parameter, or a SQLite named one.
    Parameter,
}

/// One lexical unit of a statement.
#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    /// The token as written.
    pub text: &'a str,
    /// The byte offset of `text` in the scanned SQL.
    pub start: usize,
    /// What the token is.
    pub kind: Kind,
    /// Whether a quote, comment or dollar quote the token opens is closed;
    /// an unclosed one runs to the end of the SQL.
    pub terminated: bool,
}

impl Token<'_> {
    /// Whether the token is one of `words`, in any case.
    pub fn is_word(&self, words: &[&str]) -> bool {
        self.kind == Kind::Word
            && words
                .iter()
                .any(|word| self.text.eq_ignore_ascii_case(word))
    }

    /// Whether the token is one of `symbols`.
    pub fn is_symbol(&self, symbols: &[&str]) -> bool {
        self.kind == Kind::Symbol && symbols.contains(&self.text)
    }

    /// The name a quoted identifier or a single-quoted string spells, as SQL
    /// names a column after it: without its quotes, each doubled closing
    /// quote single. Any other token as written; `None` for a quoted one its
    /// closing quote does not end.
    pub fn unquoted(&self) -> Option<String> {
        if self.kind != Kind::Identifier
            && !(self.kind == Kind::Literal && self.text.starts_with('\''))
        {
            return Some(self.text.to_string());
        }
        let close = if self.text.starts_with('[') {
            ']'
        } else {
            self.text.chars().next()?
        };
        let inner = self.text.strip_suffix(close)?.get(1..)?;
        Some(inner.replace(&format!("{close}{close}"), &close.to_string()))
    }
}

/// The tokens of `sql` under `dialect`'s rules, lexed only as far as the
/// caller reads.
pub fn tokens(sql: &str, dialect: Dialect) -> impl Iterator<Item = Token<'_>> {
    let mut offset = 0;
    std::iter::from_fn(move || {
        let rest = &sql[offset..];
        let first = rest.chars().next()?;
        let (len, kind, terminated) = token_at(sql, offset, first, dialect);
        let token = Token {
            text: &rest[..len],
            start: offset,
            kind,
            terminated,
        };
        offset += len;
        Some(token)
    })
}

/// The length, kind and termination of the token starting with `first` at
/// `offset` in `sql`.
fn token_at(sql: &str, offset: usize, first: char, dialect: Dialect) -> (usize, Kind, bool) {
    let rest = &sql[offset..];
    let whitespace = |c: char| c.is_whitespace() || (dialect == Dialect::Sqlite && c == '\u{feff}');
    if whitespace(first) {
        let len = rest.find(|c: char| !whitespace(c)).unwrap_or(rest.len());
        return (len, Kind::Space, true);
    }
    let line_comment = match dialect {
        Dialect::MySql => {
            first == '#'
                || (rest.starts_with("--")
                    && rest
                        .as_bytes()
                        .get(2)
                        .is_none_or(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control()))
        }
        _ => rest.starts_with("--"),
    };
    if line_comment {
        let len = rest.find(['\n', '\r']).unwrap_or(rest.len());
        return (len, Kind::Comment, true);
    }
    if rest.starts_with("/*") {
        let nested = matches!(dialect, Dialect::Any | Dialect::Postgres);
        let (len, terminated) = block_comment(rest, nested);
        return (len, Kind::Comment, terminated);
    }
    let brackets = matches!(dialect, Dialect::Any | Dialect::Sqlite);
    if matches!(first, '\'' | '"' | '`') || (first == '[' && brackets) {
        let kind = if first == '\'' {
            Kind::Literal
        } else {
            Kind::Identifier
        };
        let mode = if backslash_escapes(sql, offset, first, dialect) {
            EscapeMode::Backslash
        } else {
            EscapeMode::Doubled
        };
        let close = if first == '[' { ']' } else { first };
        let (len, run) = quoted_end(
            rest.char_indices().skip(1),
            close,
            mode,
            rest.len(),
            char::len_utf8,
        );
        return (len, kind, run == QuotedRun::Closed);
    }
    if dialect == Dialect::Sqlite && matches!(first, '$' | ':' | '@' | '#') {
        return (sqlite_parameter_len(rest), Kind::Parameter, true);
    }
    if first == '$' && matches!(dialect, Dialect::Any | Dialect::Postgres) {
        if let Some(open) = dollar_tag_len(rest) {
            let delimiter = &rest[..open];
            match rest[open..].find(delimiter) {
                Some(close) => return (open + close + delimiter.len(), Kind::Literal, true),
                None if dialect == Dialect::Postgres => return (rest.len(), Kind::Literal, false),
                None => {}
            }
        }
        let len = 1 + rest[1..].bytes().take_while(u8::is_ascii_digit).count();
        let kind = if len > 1 {
            Kind::Parameter
        } else {
            Kind::Symbol
        };
        return (len, kind, true);
    }
    if first == '_' || first.is_alphabetic() {
        let len = rest
            .find(|c: char| !identifier_char(c))
            .unwrap_or(rest.len());
        return (len, Kind::Word, true);
    }
    (first.len_utf8(), Kind::Symbol, true)
}

/// Whether a backslash escapes the next character inside the quote opening
/// at `quote_index`: in MySQL's strings (not its backtick identifiers), and
/// in a PostgreSQL `E'..'` string.
fn backslash_escapes(sql: &str, quote_index: usize, quote: char, dialect: Dialect) -> bool {
    match dialect {
        Dialect::MySql => quote != '`',
        Dialect::Sqlite => false,
        Dialect::Any | Dialect::Postgres => {
            quote == '\''
                && quote_index > 0
                && matches!(sql.as_bytes()[quote_index - 1], b'e' | b'E')
                && !sql[..quote_index - 1]
                    .chars()
                    .next_back()
                    .is_some_and(identifier_char)
        }
    }
}

/// SQLite named parameters can include Tcl-style namespaces and a
/// parenthesized suffix. A semicolon in that suffix belongs to the
/// parameter, not a statement.
fn sqlite_parameter_len(sql: &str) -> usize {
    let mut pos = 1;
    let mut has_name = false;
    while let Some(ch) = sql[pos..].chars().next() {
        if identifier_char(ch) || !ch.is_ascii() {
            has_name = true;
            pos += ch.len_utf8();
        } else if sql[pos..].starts_with("::") {
            pos += 2;
        } else {
            if ch == '(' && has_name {
                pos += 1;
                for ch in sql[pos..].chars() {
                    if ch.is_ascii_whitespace() {
                        break;
                    }
                    pos += ch.len_utf8();
                    if ch == ')' {
                        break;
                    }
                }
            }
            break;
        }
    }
    pos
}

/// Whether `c` can continue an unquoted identifier.
pub fn identifier_char(c: char) -> bool {
    c == '_' || c == '$' || c.is_alphanumeric()
}

/// The length of the block comment opening `sql`, and whether it is closed.
fn block_comment(sql: &str, nested: bool) -> (usize, bool) {
    let mut depth = 1;
    let mut pos = 2;
    while let Some(ch) = sql.get(pos..).and_then(|rest| rest.chars().next()) {
        if nested && sql[pos..].starts_with("/*") {
            depth += 1;
            pos += 2;
        } else if sql[pos..].starts_with("*/") {
            depth -= 1;
            pos += 2;
            if depth == 0 {
                return (pos, true);
            }
        } else {
            pos += ch.len_utf8();
        }
    }
    (pos, false)
}

/// The length of the block comment opening `sql`, to the end of `sql` when
/// it is not closed.
pub fn block_comment_len(sql: &str, nested: bool) -> usize {
    block_comment(sql, nested).0
}

/// How a quoted run escapes its closing quote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscapeMode {
    /// Only by doubling it.
    Doubled,
    /// By doubling it or with a backslash.
    Backslash,
    /// By doubling it; a backslash before it is ambiguous between backends.
    RejectAmbiguous,
}

/// How a quoted run ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotedRun {
    /// At its closing quote.
    Closed,
    /// At the end of the input, without a closing quote.
    Unterminated,
    /// At a backslash before a quote, under [`EscapeMode::RejectAmbiguous`].
    AmbiguousEscape,
}

/// Scan after an opening delimiter. Positions may be bytes or character indices;
/// callers supply the corresponding width and end so validation stays allocation-free.
pub fn quoted_end(
    chars: impl Iterator<Item = (usize, char)>,
    close: char,
    mode: EscapeMode,
    end: usize,
    width: impl Fn(char) -> usize,
) -> (usize, QuotedRun) {
    let mut chars = chars.peekable();
    while let Some((index, ch)) = chars.next() {
        if ch == '\\' && mode != EscapeMode::Doubled {
            if mode == EscapeMode::RejectAmbiguous {
                if chars.peek().is_some_and(|(_, next)| *next == close) {
                    return (index, QuotedRun::AmbiguousEscape);
                }
                if chars.peek().is_some_and(|(_, next)| *next == '\\') {
                    chars.next();
                }
            } else {
                chars.next();
            }
        } else if ch == close {
            if chars.peek().is_some_and(|(_, next)| *next == close) {
                chars.next();
            } else {
                return (index + width(ch), QuotedRun::Closed);
            }
        }
    }
    (end, QuotedRun::Unterminated)
}

/// The length of the quoted run `sql` opens with `delimiter` (`[` closes
/// with `]`), to the end of `sql` when it is not closed.
pub fn quoted_len(sql: &str, delimiter: char, backslash: bool) -> usize {
    let close = if delimiter == '[' { ']' } else { delimiter };
    let mode = if backslash {
        EscapeMode::Backslash
    } else {
        EscapeMode::Doubled
    };
    quoted_end(
        sql.char_indices().skip(1),
        close,
        mode,
        sql.len(),
        char::len_utf8,
    )
    .0
}

/// The length of the PostgreSQL dollar-quote delimiter opening `sql`, `$$` or
/// `$tag$`, whose tag does not start with a digit.
pub fn dollar_tag_len(sql: &str) -> Option<usize> {
    let end = sql[1..].find('$')? + 1;
    let tag = &sql[1..end];
    (!tag.starts_with(|c: char| c.is_ascii_digit())
        && tag.chars().all(|c| c == '_' || c.is_alphanumeric()))
    .then_some(end + 1)
}

#[cfg(test)]
#[path = "../tests/unit/lexer_tests.rs"]
mod tests;
