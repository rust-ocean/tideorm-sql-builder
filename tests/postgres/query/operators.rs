use super::*;

#[test]
fn select_pgtrgm_operators() {
    for (oper, sql) in [
        (PgBinOper::Similarity, "%"),
        (PgBinOper::WordSimilarity, "<%"),
        (PgBinOper::StrictWordSimilarity, "<<%"),
        (PgBinOper::SimilarityDistance, "<->"),
        (PgBinOper::WordSimilarityDistance, "<<->"),
        (PgBinOper::StrictWordSimilarityDistance, "<<<->"),
    ] {
        assert_eq!(
            Query::select()
                .expr(Expr::col("name").binary(oper, Expr::value("serif")))
                .from("font")
                .to_string(PostgresQueryBuilder)
                .unwrap(),
            format!(r#"SELECT "name" {sql} 'serif' FROM "font""#)
        );
    }
}

#[test]
fn select_custom_operator() {
    assert_eq!(
        Query::select()
            .expr(Expr::col("name").binary(BinOper::Custom("~*"), Expr::value("serif")))
            .from("font")
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "name" ~* 'serif' FROM "font""#
    );
    assert_eq!(
        Query::select()
            .expr(Expr::col("name").binary(BinOper::Custom("~"), Expr::value("serif")))
            .from("font")
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "name" ~ 'serif' FROM "font""#
    );
}

#[test]
fn union_1() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .union(
                UnionType::Distinct,
                Query::select()
                    .column("character")
                    .from("character")
                    .left_join(
                        "font",
                        Expr::col(("character", "font_id")).equals(("font", "id"))
                    )
                    .order_by(("font", "id"), Order::Asc)
                    .take()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"SELECT "character" FROM "character" UNION (SELECT "character" FROM "character""#,
            r#"LEFT JOIN "font" ON "character"."font_id" = "font"."id" ORDER BY "font"."id" ASC)"#
        ]
        .join(" ")
    );
}

#[test]
fn sub_query_with_fn() {
    #[derive(Iden)]
    #[iden = "ARRAY"]
    pub struct ArrayFunc;

    let sub_select = Query::select()
        .column(Asterisk)
        .from("character")
        .to_owned();

    let select = Query::select()
        .expr(Func::cust(ArrayFunc).arg(Expr::SubQuery(None, Box::new(sub_select.into()))))
        .to_owned();

    assert_eq!(
        select.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT ARRAY((SELECT * FROM "character"))"#
    );
}

#[test]
fn md5_fn() {
    assert_eq!(
        Query::select()
            .expr(Func::md5(Expr::val("test")))
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT MD5('test')"#
    );
}

#[test]
fn where_postgres_binary_operators() {
    for (oper, sql) in [
        (PgBinOper::Contains, "@>"),
        (PgBinOper::Contained, "<@"),
        (PgBinOper::Overlap, "&&"),
        (PgBinOper::GetJsonField, "->"),
        (PgBinOper::CastJsonField, "->>"),
        (PgBinOper::Regex, "~"),
        (PgBinOper::RegexCaseInsensitive, "~*"),
    ] {
        assert_eq!(
            Query::select()
                .column("character")
                .from("character")
                .and_where(Expr::col("character").binary(oper, Expr::val("test")))
                .build(PostgresQueryBuilder)
                .unwrap(),
            (
                format!(r#"SELECT "character" FROM "character" WHERE "character" {sql} $1"#),
                Values(vec!["test".into()])
            )
        );
    }
}

#[test]
fn test_issue_674_nested_logical() {
    let t = Expr::Value(true.into());
    let f = Expr::Value(false.into());

    let x_op_y = |x, op, y| Expr::Binary(Box::new(x), op, Box::new(y));
    let t_or_t = x_op_y(t.clone(), BinOper::Or, t.clone());
    let t_or_t_or_f = x_op_y(t_or_t, BinOper::Or, f);
    let t_or_t_or_f_and_t = x_op_y(t_or_t_or_f.clone(), BinOper::And, t);

    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(t_or_t_or_f_and_t)
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "character" FROM "character" WHERE (TRUE OR TRUE OR FALSE) AND TRUE"#
    );
}

#[test]
fn test_issue_674_nested_comparison() {
    let int100 = Expr::Value(100i32.into());
    let int0 = Expr::Value(0i32.into());
    let int1 = Expr::Value(1i32.into());

    let x_op_y = |x, op, y| Expr::Binary(Box::new(x), op, Box::new(y));
    let t_smaller_than_t = x_op_y(int100, BinOper::SmallerThan, int0);
    let t_smaller_than_t_smaller_than_f = x_op_y(t_smaller_than_t, BinOper::SmallerThan, int1);

    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(t_smaller_than_t_smaller_than_f)
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "character" FROM "character" WHERE (100 < 0) < 1"#
    );
}

#[test]
fn test_issue_674_and_inside_not() {
    let t = Expr::Value(true.into());
    let f = Expr::Value(false.into());

    let op_x = |op, x| Expr::Unary(op, Box::new(x));
    let x_op_y = |x, op, y| Expr::Binary(Box::new(x), op, Box::new(y));
    let f_and_t = x_op_y(f, BinOper::And, t);
    let not_f_and_t = op_x(UnOper::Not, f_and_t);

    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(not_f_and_t)
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "character" FROM "character" WHERE NOT (FALSE AND TRUE)"#
    );
}

#[test]
fn test_issue_674_nested_logical_panic() {
    let e = Expr::from(true).and(Expr::from(true).and(true).and(true));

    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(e)
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT "character" FROM "character" WHERE TRUE AND (TRUE AND TRUE AND TRUE)"#
    );
}
