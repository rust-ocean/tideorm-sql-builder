use super::*;

/// `jsonb_set` would read an index step of an object as a key and fail on a
/// member step of an array, so each step's parent is checked first and a
/// wrong one leaves the document as it is.
#[test]
fn json_set_checks_each_parent_first() {
    let path = [JsonPathStep::Member("a".into()), JsonPathStep::Element(1)];
    let (sql, values) = PostgresQueryBuilder
        .json_set(r#""doc""#, &path, &serde_json::json!(5))
        .build(&PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"CASE WHEN jsonb_typeof(("doc")::jsonb #> $1::text[]) = 'object' AND jsonb_typeof(("doc")::jsonb #> $2::text[]) = 'array' THEN jsonb_set(("doc")::jsonb, $3::text[], CAST($4 AS jsonb)) ELSE ("doc")::jsonb END"#
    );
    let texts: Vec<Value> = ["{}", r#"{"a"}"#, r#"{"a","1"}"#, "5"]
        .iter()
        .map(|text| Value::String(Some(text.to_string())))
        .collect();
    assert_eq!(values, texts);
}
