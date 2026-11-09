use super::*;

#[test]
fn select_61() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .and_where(Expr::col("character").like(LikeExpr::new("A").escape('\\')))
            .build(PostgresQueryBuilder)
            .unwrap(),
        (
            r#"SELECT "character" FROM "character" WHERE "character" LIKE $1 ESCAPE E'\\'"#
                .to_owned(),
            Values(vec!["A".into()])
        )
    );
}

#[test]
fn select_61_custom_values_numbered_placeholders_after_escape_string() {
    let (statement, values) = Query::select()
        .column("character")
        .from("character")
        .and_where(Expr::col("font_size").eq(3))
        .and_where(
            Expr::cust_with_values(
                r#"(
                "character" ILIKE '%' || $1 || '%' ESCAPE '\'
                OR "ascii" ILIKE '%' || $1 || '%' ESCAPE '\'
            )"#,
                ["needle"],
                &PostgresQueryBuilder,
            )
            .unwrap(),
        )
        .build(PostgresQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "character" FROM "character" WHERE "font_size" = $1 AND ((
                "character" ILIKE '%' || $2 || '%' ESCAPE '\'
                OR "ascii" ILIKE '%' || $3 || '%' ESCAPE '\'
            ))"#
    );
    assert_eq!(
        values,
        Values(vec![3i32.into(), "needle".into(), "needle".into()])
    );
}

#[test]
fn select_62() {
    let select = SelectStatement::new()
        .column(Asterisk)
        .from_values([(1i32, "hello"), (2, "world")], "x")
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .table_name("cte")
        .to_owned();
    let with_clause = WithClause::new().cte(cte).to_owned();
    let select = SelectStatement::new()
        .columns(["column1", "column2"])
        .from("cte")
        .to_owned();
    let query = select.with(with_clause);
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"WITH "cte" AS"#,
            r#"(SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS "x")"#,
            r#"SELECT "column1", "column2""#,
            r#"FROM "cte""#,
        ]
        .join(" ")
    );
}

#[test]
fn select_63() {
    let select = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("glyph")
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .table_name("cte")
        .to_owned();
    let query = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("cte")
        .with_cte(cte)
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"WITH "cte" AS"#,
            r#"(SELECT "id", "image", "aspect""#,
            r#"FROM "glyph")"#,
            r#"SELECT "id", "image", "aspect" FROM "cte""#,
        ]
        .join(" ")
    );
}

#[test]
fn select_64() {
    let rows = vec![
        Values(vec![
            Value::Int(Some(1)),
            Value::String(Some("glyph1.jpg".to_owned())),
            Value::Char(Some('l')),
        ]),
        Values(vec![
            Value::Int(Some(2)),
            Value::String(Some("glyph2.jpg".to_owned())),
            Value::Char(Some('p')),
        ]),
    ];

    let cte = CommonTableExpression::new()
        .values(rows)
        .columns(["id", "image", "aspect"])
        .table_name("cte")
        .to_owned();
    let with_clause = WithClause::new().cte(cte).to_owned();
    let select = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("cte")
        .to_owned();
    let query = select.with(with_clause);
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"WITH "cte" ("id", "image", "aspect") AS"#,
            r#"(VALUES (1, 'glyph1.jpg', 'l'), (2, 'glyph2.jpg', 'p'))"#,
            r#"SELECT "id", "image", "aspect" FROM "cte""#,
        ]
        .join(" ")
    );
}

#[test]
fn select_65() {
    let query = Query::select()
        .from("font")
        .columns(["id", "name"])
        .into_table(SelectInto::table("font_copy").modifier(SelectIntoTableModifier::Temporary))
        .to_owned();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"SELECT "id", "name" INTO TEMPORARY TABLE "font_copy" FROM "font""#
    );
}

#[test]
fn select_66() {
    assert_eq!(
        Query::select()
            .from("character")
            .expr(
                Expr::col("character")
                    .max()
                    .over(WindowStatement::partition_by("font_size"))
                    .alias("C"),
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT MAX("character") OVER ( PARTITION BY "font_size" ) AS "C" FROM "character""#
    );
}

#[test]
fn select_67() {
    assert_eq!(
        Query::select()
            .from("character")
            .expr(Expr::col("character").max().over("w"))
            .window("w", WindowStatement::partition_by("font_size"))
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"SELECT MAX("character") OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    );
}
