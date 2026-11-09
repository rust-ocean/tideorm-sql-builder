use crate::*;

#[test]
fn test_where_case_eq() {
    let case_statement: Expr = Expr::case(Expr::col("col").lt(5), Expr::col("othercol"))
        .finally(Expr::col("finalcol"))
        .into();

    let result = Query::select()
        .column(Asterisk)
        .from("tbl")
        .and_where(case_statement.eq(10))
        .to_string(PostgresQueryBuilder)
        .unwrap();
    assert_eq!(
        result,
        r#"SELECT * FROM "tbl" WHERE (CASE WHEN ("col" < 5) THEN "othercol" ELSE "finalcol" END) = 10"#
    );
}
