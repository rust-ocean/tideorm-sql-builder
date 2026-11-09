use crate::{EscapeBuilder, PostgresQueryBuilder};

/// Every ASCII character escapes as PostgreSQL's backslash table writes it: the named
/// controls by name, the other controls (DEL included) as three octal digits, quote and
/// backslash with a backslash, and the rest as themselves.
/// <https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-BACKSLASH-TABLE>
#[test]
fn ascii_escapes_by_the_backslash_table() {
    let named = [
        ('\0', r"\0"),
        ('\x08', r"\b"),
        ('\t', r"\t"),
        ('\n', r"\n"),
        ('\x0C', r"\f"),
        ('\r', r"\r"),
        ('\\', r"\\"),
        ('\'', r"\'"),
    ];

    for byte in 0_u8..=127 {
        let c = char::from(byte);
        let plain = c.to_string();
        let expected = match named.iter().find(|(name, _)| *name == c) {
            Some((_, escaped)) => escaped.to_string(),
            None if c.is_ascii_control() => format!("\\{byte:03o}"),
            None => plain.clone(),
        };

        assert_eq!(
            PostgresQueryBuilder.escape_string(&plain),
            expected,
            "{byte:#04x}"
        );
        assert_eq!(
            PostgresQueryBuilder.needs_escape(&plain),
            expected != plain,
            "{byte:#04x}"
        );
    }
}

#[test]
fn a_string_escapes_character_by_character() {
    let cases = [
        ("it's\ta \\ b\x7f", r"it\'s\ta \\ b\177"),
        ("\x01\x1f", r"\001\037"),
        ("say \"hi\"", "say \"hi\""),
        ("é 日本 😀", "é 日本 😀"),
        ("", ""),
    ];

    for (raw, escaped) in cases {
        assert_eq!(PostgresQueryBuilder.escape_string(raw), escaped, "{raw:?}");
        assert_eq!(
            PostgresQueryBuilder.needs_escape(raw),
            raw != escaped,
            "{raw:?}"
        );
    }
}
