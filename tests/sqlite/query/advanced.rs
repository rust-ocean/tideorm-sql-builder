use super::*;

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
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        // SQLite reads an operand's ORDER BY as the compound's, so an ordered operand is a derived table.
        [
            r#"SELECT "character" FROM "character" UNION SELECT * FROM (SELECT "character" FROM "character""#,
            r#"LEFT JOIN "font" ON "character"."font_id" = "font"."id" ORDER BY "font"."id" ASC)"#,
            r#"AS "tideorm_union_operand""#
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
        select.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT ARRAY((SELECT * FROM "character"))"#
    );
}

#[test]
fn recursive_with_multiple_ctes() {
    let sub_select1 = Query::select()
        .column(Asterisk)
        .from("character")
        .to_owned();
    let sub_select1_name = "sub1";
    let mut sub_select1_cte = CommonTableExpression::new();
    sub_select1_cte.table_name(sub_select1_name);
    sub_select1_cte.column("a");
    sub_select1_cte.query(sub_select1);
    let sub_select2 = Query::select()
        .column(Asterisk)
        .from("character")
        .to_owned();
    let sub_select2_name = "sub2";
    let mut sub_select2_cte = CommonTableExpression::new();
    sub_select2_cte.table_name(sub_select2_name);
    sub_select2_cte.column("b");
    sub_select2_cte.query(sub_select2);

    let mut with = WithClause::new();
    with.recursive(true)
        .cte(sub_select1_cte)
        .cte(sub_select2_cte);

    let mut main_sel2 = Query::select();
    main_sel2.expr(Expr::col(Asterisk)).from(sub_select2_name);
    let mut main_sel1 = Query::select();
    main_sel1
        .expr(Expr::col(Asterisk))
        .from(sub_select1_name)
        .union(UnionType::All, main_sel2);

    let query = with.query(main_sel1);

    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"WITH RECURSIVE "sub1" ("a") AS (SELECT * FROM "character"), "sub2" ("b") AS (SELECT * FROM "character") SELECT * FROM "sub1" UNION ALL SELECT * FROM "sub2""#
    );
}
