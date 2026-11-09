use super::LONG_LIST;
use crate::*;

/// The WHERE clause `builder` writes for `"id"` in (or not in) `values`, and
/// how many values it binds.
fn where_clause<B: QueryBuilder + SqlDialect>(
    builder: B,
    values: Vec<Value>,
    negated: bool,
) -> (String, usize) {
    let condition = builder.in_list(Expr::col("id"), values, negated);
    let (sql, values) = Query::select()
        .column("id")
        .from("t")
        .and_where(condition)
        .build(builder)
        .unwrap();
    let clause = sql.split_once(" WHERE ").unwrap().1.to_string();
    (clause, values.0.len())
}

fn integers(count: i64) -> Vec<Value> {
    (1..=count).map(Value::from).collect()
}

fn texts(count: usize) -> Vec<Value> {
    (0..count).map(|i| Value::from(format!("t{i}"))).collect()
}

#[test]
fn a_short_list_binds_every_value() {
    assert_eq!(
        where_clause(PostgresQueryBuilder, integers(2), false),
        (r#""id" IN ($1, $2)"#.to_string(), 2)
    );
    assert_eq!(
        where_clause(SqliteQueryBuilder, texts(2), true),
        (r#""id" NOT IN (?, ?)"#.to_string(), 2)
    );
}

#[test]
fn a_long_integer_list_is_written_as_literals_without_native_arrays() {
    for (clause, bound) in [
        where_clause(MysqlQueryBuilder, integers(LONG_LIST as i64 + 1), false),
        where_clause(SqliteQueryBuilder, integers(LONG_LIST as i64 + 1), true),
    ] {
        assert!(clause.contains("IN (1, 2, 3,"), "{clause}");
        assert_eq!(bound, 0);
    }
}

#[test]
fn a_long_list_of_one_type_is_one_postgres_array() {
    assert_eq!(
        where_clause(PostgresQueryBuilder, integers(LONG_LIST as i64 + 1), false),
        (r#""id" = ANY($1)"#.to_string(), 1)
    );
    assert_eq!(
        where_clause(PostgresQueryBuilder, texts(LONG_LIST + 1), true),
        (r#""id" <> ALL($1)"#.to_string(), 1)
    );
}

#[test]
fn a_long_text_list_is_one_json_value_on_sqlite() {
    assert_eq!(
        where_clause(SqliteQueryBuilder, texts(LONG_LIST + 1), true),
        (
            r#""id" NOT IN (SELECT value FROM json_each(?))"#.to_string(),
            1
        )
    );
    // MySQL binds every text.
    assert_eq!(
        where_clause(MysqlQueryBuilder, texts(LONG_LIST + 1), false).1,
        LONG_LIST + 1
    );
}
