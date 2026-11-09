use super::*;

#[test]
fn inject_parameters_5() {
    assert_eq!(
        inject_parameters(
            "WHERE A = $1 AND C = $2",
            &["B".into(), "D".into()],
            &PostgresQueryBuilder
        )
        .unwrap(),
        "WHERE A = 'B' AND C = 'D'"
    );
}

#[test]
fn inject_parameters_6() {
    assert_eq!(
        inject_parameters(
            "WHERE A = $2 AND C = $1",
            &["B".into(), "D".into()],
            &PostgresQueryBuilder
        )
        .unwrap(),
        "WHERE A = 'D' AND C = 'B'"
    );
}

#[test]
fn inject_parameters_7() {
    assert_eq!(
        inject_parameters("WHERE A = $1", &[Value::from("B'C")], &PostgresQueryBuilder).unwrap(),
        "WHERE A = E'B\\'C'"
    );
}

#[test]
fn markers_inside_postgres_quotes_are_text() {
    assert_eq!(
        inject_parameters(
            r"E'\'$1' || $tag$ $2 $tag$ || $1 -- $2",
            &[1.into()],
            &PostgresQueryBuilder
        )
        .unwrap(),
        r"E'\'$1' || $tag$ $2 $tag$ || 1 -- $2"
    );
    for (sql, index) in [
        ("$0", "0"),
        ("$99999999999999999999999", "99999999999999999999999"),
    ] {
        assert_eq!(
            inject_parameters(sql, &[1.into()], &PostgresQueryBuilder),
            Err(RenderError::InvalidIndex(index.to_owned()))
        );
    }
}
