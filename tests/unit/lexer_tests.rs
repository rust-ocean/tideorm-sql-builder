use super::{Dialect, Kind, tokens};

fn kinds(sql: &str, dialect: Dialect) -> Vec<(&str, Kind, bool)> {
    tokens(sql, dialect)
        .filter(|token| token.kind != Kind::Space)
        .map(|token| (token.text, token.kind, token.terminated))
        .collect()
}

#[test]
fn mysql_strings_escape_with_backslashes_but_identifiers_do_not() {
    assert_eq!(
        kinds(r"SELECT '\';' AS `a\`; # ;", Dialect::MySql),
        [
            ("SELECT", Kind::Word, true),
            (r"'\';'", Kind::Literal, true),
            ("AS", Kind::Word, true),
            (r"`a\`", Kind::Identifier, true),
            (";", Kind::Symbol, true),
            ("# ;", Kind::Comment, true),
        ]
    );
}

#[test]
fn mysql_double_minus_is_a_comment_only_before_whitespace() {
    assert_eq!(
        kinds("1--2 -- c", Dialect::MySql),
        [
            ("1", Kind::Symbol, true),
            ("-", Kind::Symbol, true),
            ("-", Kind::Symbol, true),
            ("2", Kind::Symbol, true),
            ("-- c", Kind::Comment, true),
        ]
    );
}

#[test]
fn reports_an_unterminated_nested_postgres_comment() {
    assert_eq!(
        kinds("/* outer /* inner */", Dialect::Postgres),
        [("/* outer /* inner */", Kind::Comment, false)]
    );
    assert_eq!(
        kinds("/* outer /* inner */", Dialect::Sqlite),
        [("/* outer /* inner */", Kind::Comment, true)]
    );
}

#[test]
fn any_reads_brackets_and_an_unclosed_dollar_quote_as_a_symbol() {
    assert_eq!(
        kinds("[a;b] $x$;", Dialect::Any),
        [
            ("[a;b]", Kind::Identifier, true),
            ("$", Kind::Symbol, true),
            ("x$", Kind::Word, true),
            (";", Kind::Symbol, true),
        ]
    );
    assert_eq!(
        kinds("$x$;", Dialect::Postgres),
        [("$x$;", Kind::Literal, false)]
    );
}
