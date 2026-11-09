#[test]
fn select_37() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(Cond::any().add(Cond::all()).add(Cond::any()))
        .build(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(statement, r#"SELECT "id" FROM "glyph" WHERE TRUE OR FALSE"#);
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_37a() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all()
                .add(Cond::all().not())
                .add(Cond::any().not())
                .not(),
        )
        .build(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ((NOT TRUE) AND (NOT FALSE))"#
    );
    assert_eq!(values.0, vec![]);
}

use super::*;

#[test]
fn select_38() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::any()
                .add(Expr::col("aspect").is_null())
                .add(Expr::col("aspect").is_not_null()),
        )
        .build(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL OR "aspect" IS NOT NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_39() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all()
                .add(Expr::col("aspect").is_null())
                .add(Expr::col("aspect").is_not_null()),
        )
        .build(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL AND "aspect" IS NOT NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_40() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(any![
            Expr::col("aspect").is_null(),
            all![Expr::col("aspect").is_not_null(), Expr::col("aspect").lt(8)]
        ])
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL OR ("aspect" IS NOT NULL AND "aspect" < 8)"#
    );
}

#[test]
fn select_41() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .exprs([Expr::col("image").max()])
            .from("glyph")
            .group_by_columns(["aspect"])
            .cond_having(any![Expr::col("aspect").gt(2)])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2"#
    );
}

#[test]
fn select_42() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all()
                .add_option(Some(Expr::col("aspect").lt(8)))
                .add(Expr::col("aspect").is_not_null()),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" < 8 AND "aspect" IS NOT NULL"#
    );
}

#[test]
fn select_43() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(Cond::all().add_option::<Expr>(None))
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(statement, r#"SELECT "id" FROM "glyph" WHERE TRUE"#);
}

#[test]
fn select_44() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::any()
                .not()
                .add_option(Some(Expr::col("aspect").lt(8))),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT "aspect" < 8"#
    );
}

#[test]
fn select_45() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::any()
                .not()
                .add_option(Some(Expr::col("aspect").lt(8)))
                .add(Expr::col("aspect").is_not_null()),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8 OR "aspect" IS NOT NULL)"#
    );
}

#[test]
fn select_46() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all()
                .not()
                .add_option(Some(Expr::col("aspect").lt(8))),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT "aspect" < 8"#
    );
}

#[test]
fn select_47() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all()
                .not()
                .add_option(Some(Expr::col("aspect").lt(8)))
                .add(Expr::col("aspect").is_not_null()),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8 AND "aspect" IS NOT NULL)"#
    );
}

#[test]
fn select_48() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all().add_option(Some(
                Expr::tuple([Expr::col("aspect"), Expr::value(100)])
                    .lt(Expr::tuple([Expr::value(8), Expr::value(100)])),
            )),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE ("aspect", 100) < (8, 100)"#
    );
}

#[test]
fn select_48a() {
    let statement = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(
            Cond::all().add_option(Some(
                Expr::tuple([Expr::col("aspect"), Expr::value(String::from("100"))])
                    .in_tuples([(8, String::from("100"))]),
            )),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE ("aspect", '100') IN ((8, '100'))"#
    );
}

#[test]
fn select_49() {
    let statement = Query::select()
        .column(Asterisk)
        .from("character")
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(statement, r#"SELECT * FROM "character""#);
}

#[test]
fn select_50() {
    let statement = Query::select()
        .column(("character", Asterisk))
        .column(("font", "name"))
        .from("character")
        .inner_join(
            "font",
            Expr::col(("character", "font_id")).equals(("font", "id")),
        )
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    )
}

#[test]
fn select_51() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by_with_nulls("image", Order::Desc, NullOrdering::First)
            .order_by_with_nulls(("glyph", "aspect"), Order::Asc, NullOrdering::Last)
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "image" DESC NULLS FIRST,"#,
            r#""glyph"."aspect" ASC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_52() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by_columns_with_nulls([
                ("id", Order::Asc, NullOrdering::First),
                ("aspect", Order::Desc, NullOrdering::Last),
            ])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "id" ASC NULLS FIRST,"#,
            r#""aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_53() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by_columns_with_nulls([
                (("glyph", "id"), Order::Asc, NullOrdering::First),
                (("glyph", "aspect"), Order::Desc, NullOrdering::Last),
            ])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."id" ASC NULLS FIRST,"#,
            r#""glyph"."aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_54() {
    let statement = Query::select()
        .column(Asterisk)
        .from("character")
        .from("font")
        .and_where(Expr::col(("font", "id")).equals(("character", "font_id")))
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    );
}

#[test]
fn select_55() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by(
                "id",
                Order::Field(Values(vec![
                    Value::Int(Some(4)),
                    Value::Int(Some(5)),
                    Value::Int(Some(1)),
                    Value::Int(Some(3))
                ]))
            )
            .order_by(("glyph", "aspect"), Order::Asc)
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY CASE"#,
            r#"WHEN "id"=4 THEN 0"#,
            r#"WHEN "id"=5 THEN 1"#,
            r#"WHEN "id"=1 THEN 2"#,
            r#"WHEN "id"=3 THEN 3"#,
            r#"ELSE 4 END,"#,
            r#""glyph"."aspect" ASC"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_56() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by(("glyph", "aspect"), Order::Asc)
            .order_by(
                "id",
                Order::Field(Values(vec![
                    Value::Int(Some(4)),
                    Value::Int(Some(5)),
                    Value::Int(Some(1)),
                    Value::Int(Some(3))
                ]))
            )
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE IFNULL("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."aspect" ASC,"#,
            r#"CASE WHEN "id"=4 THEN 0"#,
            r#"WHEN "id"=5 THEN 1"#,
            r#"WHEN "id"=1 THEN 2"#,
            r#"WHEN "id"=3 THEN 3"#,
            r#"ELSE 4 END"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_57() {
    let query = Query::select()
        .expr_as(
            CaseStatement::new()
                .case(Expr::col(("glyph", "aspect")).gt(0), "positive")
                .case(Expr::col(("glyph", "aspect")).lt(0), "negative")
                .finally("zero"),
            "polarity",
        )
        .from("glyph")
        .to_owned();

    assert_eq!(
        query.to_string(SqliteQueryBuilder).unwrap(),
        r#"SELECT (CASE WHEN ("glyph"."aspect" > 0) THEN 'positive' WHEN ("glyph"."aspect" < 0) THEN 'negative' ELSE 'zero' END) AS "polarity" FROM "glyph""#
    );
}

#[test]
fn select_58() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .and_where(Expr::col("character").like(LikeExpr::new("A").escape('\\')))
            .build(SqliteQueryBuilder)
            .unwrap(),
        (
            r#"SELECT "character" FROM "character" WHERE "character" LIKE ? ESCAPE '\'"#.to_owned(),
            Values(vec!["A".into()])
        )
    );
}

#[test]
fn select_59() {
    assert_eq!(
        Query::select()
            .from("character")
            .expr(
                Expr::col("character")
                    .max()
                    .over(WindowStatement::partition_by("font_size"))
                    .alias("C"),
            )
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"SELECT MAX("character") OVER ( PARTITION BY "font_size" ) AS "C" FROM "character""#
    );
}

#[test]
fn select_60() {
    assert_eq!(
        Query::select()
            .from("character")
            .expr(Expr::col("character").max().over("w"))
            .window("w", WindowStatement::partition_by("font_size"))
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"SELECT MAX("character") OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    );
}

#[test]
fn where_sqlite_binary_operators() {
    for (oper, sql) in [
        (SqliteBinOper::Glob, "GLOB"),
        (SqliteBinOper::Match, "MATCH"),
        (SqliteBinOper::GetJsonField, "->"),
        (SqliteBinOper::CastJsonField, "->>"),
    ] {
        assert_eq!(
            Query::select()
                .column("character")
                .from("character")
                .and_where(Expr::col("character").binary(oper, Expr::val("test")))
                .build(SqliteQueryBuilder)
                .unwrap(),
            (
                format!(r#"SELECT "character" FROM "character" WHERE "character" {sql} ?"#),
                Values(vec!["test".into()])
            )
        );
    }
}
