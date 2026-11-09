use crate::*;

/// `mutation_limits`: whether the backend has `UPDATE`/`DELETE .. LIMIT`; PostgreSQL refuses it.
fn limits(builder: impl QueryBuilder, mutation_limits: bool) {
    let select = Query::select()
        .expr(Expr::val(" LIMIT "))
        .limit(7)
        .offset(2)
        .to_owned();
    let (sql, values) = select.build_any(&builder).unwrap();
    assert!(sql.contains(" LIMIT 7 OFFSET "));
    assert_eq!(values, Values(vec![" LIMIT ".into(), 2u64.into()]));
    let update = Query::update()
        .table("rows")
        .value("n", 8)
        .limit(9)
        .to_owned();
    if mutation_limits {
        let (sql, values) = update.build_any(&builder).unwrap();
        assert!(sql.ends_with(" LIMIT 9"));
        assert_eq!(values, Values(vec![8.into()]));
    } else {
        assert!(update.build_any(&builder).is_err());
    }
    let delete = Query::delete()
        .from_table("rows")
        .and_where(Expr::col("n").eq(8))
        .limit(0)
        .to_owned();
    if mutation_limits {
        let (sql, values) = delete.build_any(&builder).unwrap();
        assert!(sql.ends_with(" LIMIT 0"));
        assert_eq!(values, Values(vec![8.into()]));
    } else {
        assert!(delete.build_any(&builder).is_err());
    }
    let nested = Query::select()
        .expr(Expr::val(1))
        .limit(u64::MAX)
        .offset(4)
        .to_owned();
    let (sql, values) = Query::select()
        .expr(Expr::col("n").in_subquery(nested))
        .limit(3)
        .build_any(&builder)
        .unwrap();
    assert!(sql.contains("LIMIT 18446744073709551615 OFFSET "));
    assert!(sql.ends_with(" LIMIT 3"));
    assert_eq!(values, Values(vec![1.into(), 4u64.into()]));
}

#[test]
fn every_limit_is_literal_and_offsets_remain_bound() {
    limits(MysqlQueryBuilder, true);
    limits(PostgresQueryBuilder, false);
    limits(SqliteQueryBuilder, true);
}

#[test]
fn active_default_rows_respects_sources_columns_and_usize_counts() {
    let mut insert = Query::insert().into_table("rows").to_owned();
    assert_eq!(insert.active_default_rows(), None);
    insert.or_default_values_many(0);
    assert_eq!(insert.active_default_rows(), None);
    insert.or_default_values_many(usize::MAX);
    assert_eq!(insert.active_default_rows(), Some(usize::MAX));
    insert.columns(["id"]);
    assert_eq!(insert.active_default_rows(), None);
    let mut insert = Query::insert()
        .into_table("rows")
        .or_default_values_many(3)
        .to_owned();
    insert.select_from(Query::select()).unwrap();
    assert_eq!(insert.active_default_rows(), None);
    let mut insert = Query::insert()
        .into_table("rows")
        .or_default_values_many(3)
        .to_owned();
    insert
        .columns(["id"])
        .values_panic([1.into()])
        .columns([] as [&str; 0]);
    assert_eq!(insert.active_default_rows(), None);
}

#[test]
fn unicode_char_literals_use_the_same_escaping_as_strings() {
    fn check(builder: impl QueryBuilder) {
        for character in ['\u{03bb}', '\u{e9}', '\'', '\u{1f642}', '\u{0141}'] {
            assert_eq!(
                values::value_to_string(&builder, &Value::Char(Some(character))).unwrap(),
                values::value_to_string(&builder, &Value::String(Some(character.to_string())))
                    .unwrap()
            );
        }
    }
    check(MysqlQueryBuilder);
    check(PostgresQueryBuilder);
    check(SqliteQueryBuilder);
}

#[test]
fn checked_parameters_reject_missing_unused_and_invalid_indexes() {
    for sql in [
        "$0",
        "$2",
        "$99999999999999999999999999999999",
        "$",
        "$nope",
    ] {
        assert!(
            inject_parameters(sql, &[1.into()], &PostgresQueryBuilder).is_err(),
            "{sql}"
        );
        assert!(
            Expr::cust_with_values(sql, [1], &PostgresQueryBuilder).is_err(),
            "{sql}"
        );
    }
    for sql in ["? + ?", "1"] {
        assert!(inject_parameters(sql, &[1.into()], &MysqlQueryBuilder).is_err());
        assert!(Expr::cust_with_values(sql, [1], &SqliteQueryBuilder).is_err());
    }
    assert!(inject_parameters("?", &[], &SqliteQueryBuilder).is_err());
    assert!(
        Expr::cust_with_exprs("$1", [Expr::val(1), Expr::val(2)], &PostgresQueryBuilder).is_err()
    );
    assert!(Expr::cust_with_expr("$0", Expr::val(1), &PostgresQueryBuilder).is_err());
    assert_eq!(
        inject_parameters("$2 + $1 + $2", &[1.into(), 2.into()], &PostgresQueryBuilder).unwrap(),
        "2 + 1 + 2"
    );
    let expr = Expr::cust_with_values("$2 + $1 + $2", [1, 2], &PostgresQueryBuilder).unwrap();
    let (sql, values) = Query::select()
        .expr(Expr::val(9))
        .expr(expr)
        .build(PostgresQueryBuilder)
        .unwrap();
    assert_eq!(sql, "SELECT $1, $2 + $3 + $4");
    assert_eq!(values, Values(vec![9.into(), 2.into(), 1.into(), 2.into()]));
}

#[test]
fn malformed_tuple_and_json_conversions_return_errors() {
    assert!(i32::from_value_tuple(ValueTuple::Two(1.into(), 2.into())).is_err());
    assert!(<(i32, i32)>::from_value_tuple(ValueTuple::One(1.into())).is_err());
    assert!(<(i32, i32)>::from_value_tuple(ValueTuple::Two(1.into(), "bad".into())).is_err());
    assert!(
        <(i32, i32, i32)>::from_value_tuple(ValueTuple::Three(1.into(), 2.into(), "bad".into()))
            .is_err()
    );
    assert!(<(i32, i32, i32, i32)>::from_value_tuple(ValueTuple::Many(vec![1.into(); 3])).is_err());
    assert!(
        <(i32, i32, i32, i32)>::from_value_tuple(ValueTuple::Many(vec!["bad".into(); 4])).is_err()
    );
    assert!(value_to_json_value(&Value::Bytes(Some(vec![255]))).is_err());
    assert_eq!(
        value_to_json_value(&Value::Bytes(Some(b"hello".to_vec()))).unwrap(),
        serde_json::json!("hello")
    );
}
#[test]
fn array_conversion_rejects_wrong_elements_without_panicking() {
    let invalid = Value::Array(
        ArrayType::Int,
        Some(Box::new(vec![1.into(), "wrong".into()])),
    );
    assert!(<Vec<i32> as ValueType>::try_from(invalid).is_err());
    let invalid = Value::Array(ArrayType::Int, Some(Box::new(vec![Value::Int(None)])));
    assert!(<Vec<i32> as ValueType>::try_from(invalid).is_err());
    let invalid_json = Value::Array(
        ArrayType::Bytes,
        Some(Box::new(vec![Value::Bytes(Some(vec![255]))])),
    );
    assert!(value_to_json_value(&invalid_json).is_err());
    assert_eq!(
        <Vec<i32> as ValueType>::try_from(vec![1, 2].into()).unwrap(),
        vec![1, 2]
    );
}

#[test]
fn a_named_window_precedes_order_by_and_limit() {
    let query = Query::select()
        .from("t")
        .expr(Expr::col("v").max().over("w"))
        .window("w", WindowStatement::partition_by("k"))
        .order_by("v", Order::Asc)
        .limit(5)
        .to_owned();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT MAX("v") OVER "w" FROM "t" WINDOW "w" AS (PARTITION BY "k") ORDER BY "v" ASC LIMIT 5"#
    );
    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        "SELECT MAX(`v`) OVER `w` FROM `t` WINDOW `w` AS (PARTITION BY `k`) ORDER BY `v` ASC LIMIT 5"
    );
    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT MAX("v") OVER "w" FROM "t" WINDOW "w" AS (PARTITION BY "k") ORDER BY "v" ASC LIMIT 5"#
    );
}
