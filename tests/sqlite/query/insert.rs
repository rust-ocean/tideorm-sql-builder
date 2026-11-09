use super::*;

#[test]
#[allow(clippy::approx_constant)]
fn insert_2() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image", "aspect"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_3() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image", "aspect"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .values_panic([Value::Double(None).into(), 2.1345.into()])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415), (NULL, 2.1345)"#
    );
}

#[test]

fn insert_4() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image"])
            .values_panic([chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .into()])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image") VALUES ('1970-01-01 00:00:00.000000')"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_9() {
    assert_eq!(
        Query::insert()
            .into_table("task")
            .columns(["is_done"])
            .values_panic([true.into()])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "task" ("is_done") VALUES (TRUE)"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_10() {
    assert_eq!(
        Query::insert()
            .into_table("task")
            .columns(["is_done"])
            .values_panic([false.into()])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "task" ("is_done") VALUES (FALSE)"#
    );
}

#[test]
fn insert_from_select() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .select_from(
                Query::select()
                    .column("aspect")
                    .column("image")
                    .from("glyph")
                    .conditions(
                        true,
                        |x| {
                            x.and_where(Expr::col("image").like("%"));
                        },
                        |x| {
                            x.and_where(Expr::col("id").eq(6));
                        },
                    )
                    .to_owned()
            )
            .unwrap()
            .to_owned()
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "glyph" WHERE "image" LIKE '%'"#
    );
}

#[test]
fn insert_from_select_2() {
    let sql = Query::insert()
        .into_table("glyph")
        .columns(["image"])
        .select_from(
            Query::select()
                .expr(Expr::val("hello"))
                .cond_where(Cond::all().not().add(Expr::exists(
                    Query::select().expr(Expr::val("world")).to_owned(),
                )))
                .to_owned(),
        )
        .unwrap()
        .to_owned()
        .to_string(SqliteQueryBuilder)
        .unwrap();

    assert_eq!(
        sql,
        r#"INSERT INTO "glyph" ("image") SELECT 'hello' WHERE NOT EXISTS(SELECT 'world')"#
    );
}

#[test]

fn insert_5() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image"])
            .values_panic([uuid::Uuid::nil().into()])
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image") VALUES ('00000000-0000-0000-0000-000000000000')"#
    );
}

#[test]
fn insert_6() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .or_default_values()
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" DEFAULT VALUES"#
    );
}

#[test]
fn insert_7() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .or_default_values()
            .returning_col("id")
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" DEFAULT VALUES RETURNING "id""#
    );
}

#[test]
fn insert_default_rows_in_one_statement() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .or_default_values_many(3)
            .returning_col("id")
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" (rowid) VALUES (NULL), (NULL), (NULL) RETURNING "id""#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_1() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "image"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(OnConflict::column("id").update_column("aspect").to_owned())
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder).unwrap(),
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
            .to_string(SqliteQueryBuilder).unwrap(),
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
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder).unwrap(),
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
            .to_string(SqliteQueryBuilder).unwrap(),
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
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder)
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
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING *"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_specific_columns() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image", "aspect"])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .returning(Query::returning().columns(["id", "image"]))
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING "id", "image""#
    );
}
