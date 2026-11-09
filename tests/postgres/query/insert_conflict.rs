use super::*;

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_1() {
    let query = Query::insert()
        .into_table("glyph")
        .columns(["aspect", "image"])
        .values_panic([
            "04108048005887010020060000204E0180400400".into(),
            3.1415.into(),
        ])
        .on_conflict(OnConflict::column("id").update_column("aspect").to_owned())
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_2() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .update_columns(["aspect", "image"])
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_3() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .values([
                        ("aspect", "04108048005887010020060000204E0180400400".into()),
                        ("image", 3.1415.into()),
                    ])
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = 3.1415"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_4() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .value("image", Expr::val(1).add(2))
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "image" = 1 + 2"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_5() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .value("aspect", Expr::val("04108048005887010020060000204E0180400400"))
                    .update_column("image")
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = "excluded"."image""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_6() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .update_column("aspect")
                    .value("image", Expr::val(1).add(2))
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = 1 + 2"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_7() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::new()
                    .expr(Expr::col("id"))
                    .update_column("aspect")
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_8() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::new()
                    .exprs([Expr::col("id"), Expr::col("aspect")])
                    .update_column("aspect")
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_9() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::column("id")
                    .expr(Func::lower(Expr::col("tokens")))
                    .update_column("aspect")
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", LOWER("tokens")) DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
fn insert_on_conflict_10() {
    assert_eq!(
        Query::insert()
            .into_table("font")
            .columns(["id", "name"])
            .values_panic([15.into(), "CyberFont Sans Serif".into()])
            .on_conflict(
                OnConflict::constraint("name_unique")
                    .do_nothing()
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "font" ("id", "name")"#,
            r#"VALUES (15, 'CyberFont Sans Serif')"#,
            r#"ON CONFLICT ON CONSTRAINT "name_unique" DO NOTHING"#,
        ]
        .join(" ")
    );
}

#[test]
fn insert_on_conflict_11() {
    assert_eq!(
        Query::insert()
            .into_table("font")
            .columns(["id", "name"])
            .values_panic([20.into(), "Monospaced terminal".into()])
            .on_conflict(
                OnConflict::column("name")
                    .expr(Expr::is_null(Expr::col("variant")))
                    .do_nothing()
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "font" ("id", "name")"#,
            r#"VALUES (20, 'Monospaced terminal')"#,
            r#"ON CONFLICT ("name", ("variant" IS NULL)) DO NOTHING"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_do_nothing() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic(["abcd".into(), 3.1415.into()])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .do_nothing()
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('abcd', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_do_nothing_on() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic(["abcd".into(), 3.1415.into()])
            .on_conflict(
                OnConflict::columns(["id", "aspect"])
                    .do_nothing_on(["id"])
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('abcd', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_all_columns() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image", "aspect"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .returning(Query::returning().all())
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING *"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_specific_columns() {
    let query = Query::insert()
        .into_table("glyph")
        .columns(["image", "aspect"])
        .values_panic([
            "04108048005887010020060000204E0180400400".into(),
            3.1415.into(),
        ])
        .returning(Query::returning().columns(["id", "image"]))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING "id", "image""#
    );
}
