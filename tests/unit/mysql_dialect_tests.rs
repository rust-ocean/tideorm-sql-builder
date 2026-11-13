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

/// `JSON_ARRAYAGG` leaves its order undefined, so the kept elements are put
/// in their original order explicitly: an `ORDER BY` in MariaDB's, a window
/// in that order for MySQL's.
#[test]
fn array_remove_keeps_the_original_order() {
    let render = |dialect: &dyn SqlDialect| {
        dialect
            .array_remove("`doc`", "", Value::from("B"))
            .build(dialect)
            .0
    };
    assert_eq!(
        render(&MysqlQueryBuilder),
        "CASE WHEN `doc` IS NULL THEN NULL WHEN JSON_TYPE(`doc`) <> 'ARRAY' THEN `doc` \
         ELSE COALESCE((SELECT JSON_ARRAYAGG(JSON_EXTRACT(tideorm_element.v, '$')) \
         OVER (ORDER BY tideorm_element.i ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) \
         FROM JSON_TABLE(`doc`, '$[*]' COLUMNS (i FOR ORDINALITY, v JSON PATH '$')) AS tideorm_element \
         WHERE NOT COALESCE(tideorm_element.v = CAST(? AS JSON), FALSE) LIMIT 1), JSON_ARRAY()) END"
    );
    assert_eq!(
        render(&MariaDbDialect),
        "CASE WHEN `doc` IS NULL THEN NULL WHEN JSON_TYPE(`doc`) <> 'ARRAY' THEN `doc` \
         ELSE (SELECT COALESCE(JSON_ARRAYAGG(JSON_EXTRACT(tideorm_element.v, '$') \
         ORDER BY tideorm_element.i), JSON_ARRAY()) \
         FROM JSON_TABLE(`doc`, '$[*]' COLUMNS (i FOR ORDINALITY, v JSON PATH '$')) AS tideorm_element \
         WHERE NOT COALESCE((CASE WHEN tideorm_element.v IS NOT NULL \
         THEN JSON_EQUALS(tideorm_element.v, ?) END), FALSE)) END"
    );
}
