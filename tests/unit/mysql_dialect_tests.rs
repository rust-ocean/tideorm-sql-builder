use super::*;

fn texts(texts: &[&str]) -> Vec<Value> {
    texts
        .iter()
        .map(|text| Value::String(Some(text.to_string())))
        .collect()
}

/// `JSON_SET` would wrap a scalar parent of an index step in an array, or
/// replace it, so each step's parent is checked first and a wrong one leaves
/// the column as it is.
#[test]
fn json_set_checks_each_parent_first() {
    let path = [JsonPathStep::Member("k".into()), JsonPathStep::Element(1)];
    for dialect in [&MysqlQueryBuilder as &dyn SqlDialect, &MariaDbDialect] {
        let (sql, values) = dialect
            .json_set("`doc`", &path, &serde_json::json!(5))
            .build(dialect);
        assert_eq!(
            sql,
            "CASE WHEN JSON_TYPE(JSON_EXTRACT(`doc`, ?)) = 'OBJECT' \
             AND JSON_TYPE(JSON_EXTRACT(`doc`, ?)) = 'ARRAY' \
             THEN JSON_SET(`doc`, ?, JSON_EXTRACT(?, '$')) ELSE `doc` END"
        );
        assert_eq!(values, texts(&["$", r#"$."k""#, r#"$."k"[1]"#, "5"]));

        let (sql, values) = dialect
            .json_set("`doc`", &[], &serde_json::json!(5))
            .build(dialect);
        assert_eq!((sql.as_str(), values.len()), ("`doc`", 0));
    }
}
