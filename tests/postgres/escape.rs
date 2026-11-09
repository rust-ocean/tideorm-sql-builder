use super::*;

#[test]
fn string_literals_escape_backslashes_quotes_and_control_characters() {
    for (raw, escaped) in [
        (r#" "abc" "#, r#" "abc" "#),
        ("a\nb\tc", "a\\nb\\tc"),
        ("a\\b", "a\\\\b"),
        ("a\"b", r#"a"b"#),
        // Other ASCII stays as it is
        ("\x08\x0C\n\r\t\0'\\ABC你😀", r"\b\f\n\r\t\0\'\\ABC你😀"),
    ] {
        assert_eq!(PostgresQueryBuilder.escape_string(raw), escaped, "{raw:?}");
    }
}
