use super::*;

#[test]
fn string_literals_escape_quotes_backslashes_and_control_characters() {
    for (raw, escaped) in [
        (r#" "abc" "#, r#" \"abc\" "#),
        ("a\nb\tc", "a\\nb\\tc"),
        ("a\\b", "a\\\\b"),
        ("a\"b", "a\\\"b"),
        ("it's", "it''s"),
    ] {
        assert_eq!(MysqlQueryBuilder.escape_string(raw), escaped, "{raw:?}");
    }
}
