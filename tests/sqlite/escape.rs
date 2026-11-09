use super::*;

#[test]
fn string_literals_only_double_their_single_quotes() {
    for (raw, escaped) in [
        (r#" "abc" "#, r#" "abc" "#),
        ("a\nb\tc", "a\nb\tc"),
        ("a\\b", "a\\b"),
        ("a\"b", "a\"b"),
        ("a'c", "a''c"),
    ] {
        assert_eq!(SqliteQueryBuilder.escape_string(raw), escaped, "{raw:?}");
    }
}
