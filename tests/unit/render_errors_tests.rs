use crate::*;

/// The error `statement` fails with, through every entry point.
fn query_error<B: QueryBuilder + Default>(
    statement: &impl QueryStatementWriter,
    builder: B,
) -> RenderError {
    let error = statement.build_any(&builder).unwrap_err();
    assert_eq!(statement.to_string(builder).unwrap_err(), error);
    assert_eq!(statement.build(B::default()).unwrap_err(), error);
    error
}

#[test]
fn rendering_stops_at_the_first_error() {
    let first = RenderError::Unsupported("first".into());
    let second = RenderError::Unsupported("second".into());
    let query = Query::select()
        .expr(Expr::Invalid(first.clone()))
        .expr(Expr::Invalid(second))
        .expr(Expr::val(17))
        .to_owned();
    assert_eq!(query_error(&query, PostgresQueryBuilder), first);
    let outer = Query::select().expr(Expr::exists(query.clone())).to_owned();
    assert_eq!(query_error(&outer, PostgresQueryBuilder), first);
    let with = WithClause::new()
        .cte(
            CommonTableExpression::new()
                .table_name("nested")
                .query(query)
                .to_owned(),
        )
        .to_owned()
        .query(Query::select().expr(Expr::val(1)).to_owned());
    assert_eq!(query_error(&with, PostgresQueryBuilder), first);
    let insert = Query::insert()
        .into_table("t")
        .columns(["v"])
        .values_panic([1.into()])
        .returning(Query::returning().expr(Expr::Invalid(first)))
        .to_owned();
    query_error(&insert, PostgresQueryBuilder);
}

#[test]
fn incomplete_with_queries_return_errors() {
    query_error(&WithQuery::new(), PostgresQueryBuilder);
    let cte = CommonTableExpression::new()
        .query(Query::select().expr(Expr::val(1)).to_owned())
        .to_owned();
    let query = WithClause::new().cte(cte).to_owned().query(Query::select());
    query_error(&query, PostgresQueryBuilder);
    let cte = CommonTableExpression::new()
        .table_name("missing_body")
        .to_owned();
    let query = WithClause::new().cte(cte).to_owned().query(Query::select());
    query_error(&query, PostgresQueryBuilder);
}

#[test]
fn unsupported_query_constructs_return_errors() {
    let query = Query::select()
        .expr(extension::postgres::PgFunc::gen_random_uuid())
        .to_owned();
    query_error(&query, MysqlQueryBuilder);
    query_error(&query, SqliteQueryBuilder);
    let query = Query::select()
        .expr(Expr::col("v").binary(
            BinOper::PgOperator(extension::postgres::PgBinOper::Contains),
            1,
        ))
        .to_owned();
    query_error(&query, MysqlQueryBuilder);
    let query = Query::select()
        .from("t")
        .join(JoinType::FullOuterJoin, "u", Expr::col("v").eq(1))
        .to_owned();
    query_error(&query, MysqlQueryBuilder);
    for op in [SubQueryOper::Any, SubQueryOper::Some, SubQueryOper::All] {
        let query = Query::select()
            .expr(Expr::SubQuery(
                Some(op),
                Box::new(Query::select().expr(Expr::val(1)).to_owned().into()),
            ))
            .to_owned();
        query_error(&query, SqliteQueryBuilder);
    }
    let mut sql = String::new();
    assert!(
        functions::prepare_join_on(&PostgresQueryBuilder, &JoinOn::Columns(vec![]), &mut sql)
            .is_err()
    );
}

#[test]
fn empty_membership_has_no_bindings_and_preserves_parameter_order() {
    fn check(builder: impl QueryBuilder, expected: &str) {
        let query = Query::select()
            .expr(Expr::col("v").is_in(Vec::<i32>::new()))
            .expr(Expr::col("v").is_not_in(Vec::<i32>::new()))
            .expr(Expr::val(9))
            .to_owned();
        let (sql, values) = query.build_any(&builder).unwrap();
        assert_eq!(sql, expected);
        assert_eq!(values, Values(vec![9.into()]));
    }
    check(PostgresQueryBuilder, "SELECT 1 = 2, 1 = 1, $1");
    check(MysqlQueryBuilder, "SELECT 1 = 2, 1 = 1, ?");
    check(SqliteQueryBuilder, "SELECT 1 = 2, 1 = 1, ?");
}
#[test]
fn malformed_enum_arrays_fail_in_parameters_literals_and_display() {
    for element in [
        Value::Int(Some(1)),
        Value::Enum(OptionEnum::None("kind".into())),
    ] {
        let value = Value::Array(
            ArrayType::Enum(Box::new("kind".into())),
            Some(Box::new(vec![element])),
        );
        query_error(
            &Query::select().expr(Expr::val(value.clone())).to_owned(),
            PostgresQueryBuilder,
        );
        query_error(
            &Query::select()
                .expr(Expr::Constant(value.clone()))
                .to_owned(),
            PostgresQueryBuilder,
        );
        assert!(inject_parameters("SELECT $1", &[value], &PostgresQueryBuilder).is_err());
    }
}

#[test]
fn clauses_a_backend_lacks_are_refused() {
    let distinct_on = Query::select()
        .from("t")
        .distinct_on(["v"])
        .column("v")
        .to_owned();
    query_error(&distinct_on, MysqlQueryBuilder);
    query_error(&distinct_on, SqliteQueryBuilder);
    let into = Query::select()
        .from("t")
        .column("v")
        .into_table(SelectInto::table("copy"))
        .to_owned();
    query_error(&into, MysqlQueryBuilder);
    query_error(&into, SqliteQueryBuilder);
    for lock in [LockType::NoKeyUpdate, LockType::KeyShare] {
        let query = Query::select().from("t").column("v").lock(lock).to_owned();
        query_error(&query, MysqlQueryBuilder);
    }

    let insert = |on_conflict: Option<OnConflict>| {
        let mut insert = Query::insert();
        insert
            .into_table("t")
            .columns(["v"])
            .values_panic([1.into()]);
        if let Some(on_conflict) = on_conflict {
            insert.on_conflict(on_conflict);
        }
        insert
    };
    let do_nothing = insert(Some(OnConflict::column("v").do_nothing().to_owned()));
    query_error(&do_nothing, MysqlQueryBuilder);
    let conditional_update = insert(Some(
        OnConflict::column("v")
            .update_column("v")
            .action_and_where(Expr::col("v").gt(0))
            .to_owned(),
    ));
    query_error(&conditional_update, MysqlQueryBuilder);
    let on_constraint = insert(Some(OnConflict::constraint("t_v").do_nothing().to_owned()));
    query_error(&on_constraint, SqliteQueryBuilder);
    query_error(&insert(None).replace().to_owned(), PostgresQueryBuilder);
    let default_value = Query::insert()
        .into_table("t")
        .columns(["v"])
        .values_panic([Expr::keyword_default()])
        .to_owned();
    query_error(&default_value, SqliteQueryBuilder);

    let mut update = Query::update()
        .table("t")
        .value("v", 1)
        .and_where(Expr::col("v").eq(0))
        .to_owned();
    query_error(&update.clone().limit(1).to_owned(), PostgresQueryBuilder);
    query_error(
        &update.clone().order_by("v", Order::Asc).to_owned(),
        PostgresQueryBuilder,
    );
    let mut delete = Query::delete()
        .from_table("t")
        .and_where(Expr::col("v").eq(0))
        .to_owned();
    query_error(&delete.clone().limit(1).to_owned(), PostgresQueryBuilder);
    query_error(
        &delete.clone().order_by("v", Order::Asc).to_owned(),
        PostgresQueryBuilder,
    );
    // Where the clause exists, it still renders.
    assert!(update.limit(1).to_string(MysqlQueryBuilder).is_ok());
    assert!(delete.limit(1).to_string(MysqlQueryBuilder).is_ok());
    assert!(do_nothing.to_string(SqliteQueryBuilder).is_ok());
    assert!(on_constraint.to_string(PostgresQueryBuilder).is_ok());
    assert!(insert(None).replace().to_string(SqliteQueryBuilder).is_ok());
}

#[test]
fn straight_join_renders_only_on_the_mysql_family() {
    let query = Query::select()
        .column("v")
        .from("t")
        .straight_join("u", Expr::col(("t", "id")).equals(("u", "id")))
        .to_owned();
    query_error(&query, PostgresQueryBuilder);
    query_error(&query, SqliteQueryBuilder);
    assert!(
        query
            .to_string(MysqlQueryBuilder)
            .unwrap()
            .contains("STRAIGHT_JOIN")
    );
}

#[test]
fn one_argument_greatest_and_least_are_the_argument() {
    let query = Query::select()
        .expr(Func::greatest([Expr::col("v")]))
        .expr(Func::least([Expr::col("v")]))
        .from("t")
        .to_owned();
    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT ("v"), ("v") FROM "t""#
    );
    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        "SELECT (`v`), (`v`) FROM `t`"
    );
    let none = Query::select()
        .expr(Func::greatest(Vec::<Expr>::new()))
        .to_owned();
    query_error(&none, PostgresQueryBuilder);
}

#[test]
fn field_order_matches_null_and_refuses_an_empty_list() {
    let order = |values: Vec<Value>| {
        Query::select()
            .column("v")
            .from("t")
            .order_by("v", Order::Field(Values(values)))
            .to_owned()
    };
    assert_eq!(
        order(vec![1.into(), Value::Int(None)])
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "v" FROM "t" ORDER BY CASE WHEN "v"=1 THEN 0 WHEN "v" IS NULL THEN 1 ELSE 2 END"#
    );
    query_error(&order(Vec::new()), PostgresQueryBuilder);
}

/// MySQL writes a shared lock `FOR SHARE`, the form that takes `OF`,
/// `NOWAIT` and `SKIP LOCKED`; MariaDB has only `LOCK IN SHARE MODE`, which
/// takes the last two, and no `OF` at all.
#[test]
fn mysql_family_locks_follow_each_server() {
    let lock = |r#type: LockType, tables: &[&'static str], behavior: LockBehavior| {
        Query::select()
            .column("v")
            .from("t")
            .lock_with_tables_behavior(r#type, tables.iter().copied(), behavior)
            .to_owned()
    };
    let shared = lock(LockType::Share, &[], LockBehavior::Nowait);
    assert_eq!(
        shared.to_string(MysqlQueryBuilder).unwrap(),
        "SELECT `v` FROM `t` FOR SHARE NOWAIT"
    );
    assert_eq!(
        shared.to_string(MariaDbDialect).unwrap(),
        "SELECT `v` FROM `t` LOCK IN SHARE MODE NOWAIT"
    );
    let skipping = lock(LockType::Update, &[], LockBehavior::SkipLocked);
    assert_eq!(
        skipping.to_string(MariaDbDialect).unwrap(),
        "SELECT `v` FROM `t` FOR UPDATE SKIP LOCKED"
    );
    for r#type in [LockType::Share, LockType::Update] {
        let of = lock(r#type, &["t"], LockBehavior::SkipLocked);
        assert!(
            of.to_string(MysqlQueryBuilder)
                .unwrap()
                .contains(" OF `t` SKIP LOCKED")
        );
        query_error(&of, MariaDbDialect);
    }
    let key_share = lock(LockType::KeyShare, &[], LockBehavior::Nowait);
    query_error(&key_share, MysqlQueryBuilder);
    query_error(&key_share, MariaDbDialect);
}

#[test]
fn aggregate_filters_render_on_postgres_and_sqlite_only() {
    let query = Query::select()
        .expr(Func::count(Expr::col("v")).filter(Cond::all().add(Expr::col("v").gt(1))))
        .from("t")
        .to_owned();
    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT COUNT("v") FILTER (WHERE "v" > 1) FROM "t""#
    );
    query_error(&query, MysqlQueryBuilder);
    query_error(&query, MariaDbDialect);
}

#[test]
fn sqlite_refuses_the_functions_it_lacks() {
    for function in [
        Func::bit_and(Expr::col("v")),
        Func::bit_or(Expr::col("v")),
        Func::md5(Expr::col("v")),
    ] {
        let query = Query::select().expr(function).from("t").to_owned();
        query_error(&query, SqliteQueryBuilder);
        assert!(query.to_string(MysqlQueryBuilder).is_ok());
        assert!(query.to_string(PostgresQueryBuilder).is_ok());
    }
}

/// `DISTINCTROW` is MySQL's synonym of `DISTINCT`, which the others write.
#[test]
fn distinct_row_is_distinct() {
    let mut query = Query::select().column("v").from("t").to_owned();
    query.distinct = Some(SelectDistinct::DistinctRow);
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT DISTINCT "v" FROM "t""#
    );
    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT DISTINCT "v" FROM "t""#
    );
    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        "SELECT DISTINCTROW `v` FROM `t`"
    );
}
