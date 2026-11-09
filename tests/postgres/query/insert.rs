use super::*;

#[test]
#[allow(clippy::approx_constant)]
fn insert_2() {
    let query = Query::insert()
        .into_table("glyph")
        .columns(["image", "aspect"])
        .values_panic([
            "04108048005887010020060000204E0180400400".into(),
            f64::consts::PI.into(),
        ])
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.141592653589793)"#
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
                f64::consts::PI.into(),
            ])
            .values_panic([Value::String(None).into(), 2.1345.into()])
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.141592653589793), (NULL, 2.1345)"#
    );
}

#[test]
fn insert_enum_literal_cast() {
    let value = sql_builder::Enum {
        type_name: "FontSizeEnum".to_owned().into(),
        value: "large".into(),
    };

    assert_eq!(
        Query::insert()
            .into_table("character")
            .columns(["font_size"])
            .values_panic([Expr::val(value)])
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "character" ("font_size") VALUES ('large'::"FontSizeEnum")"#
    );
}

#[test]
fn insert_enum_array_param_cast() {
    let value = Value::Array(
        ArrayType::Enum(Box::new("FontSizeEnum".to_owned().into())),
        Some(Box::new(vec![
            sql_builder::Enum {
                type_name: "FontSizeEnum".to_owned().into(),
                value: "large".into(),
            }
            .into(),
        ])),
    );

    let (statement, values) = Query::insert()
        .into_table("character")
        .columns(["font_size"])
        .values_panic([Expr::val(value)])
        .build(PostgresQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r#"INSERT INTO "character" ("font_size") VALUES ($1::"FontSizeEnum"[])"#
    );
    assert_eq!(values, Values(vec![vec!["large".to_owned()].into()]));
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
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        "INSERT INTO \"glyph\" (\"image\") VALUES ('1970-01-01 00:00:00.000000')"
    );
}

#[test]

fn insert_5() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image"])
            .values_panic([uuid::Uuid::nil().into()])
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        "INSERT INTO \"glyph\" (\"image\") VALUES ('00000000-0000-0000-0000-000000000000')"
    );
}

#[test]
fn insert_from_select() {
    let query = Query::insert()
        .into_table("glyph")
        .or_default_values()
        .columns(["aspect", "image"])
        .select_from(
            Query::select()
                .column("aspect")
                .column("image")
                .from("font")
                .conditions(
                    true,
                    |x| {
                        x.and_where(Expr::col("image").like("%"));
                    },
                    |x| {
                        x.and_where(Expr::col("id").eq(6));
                    },
                )
                .to_owned(),
        )
        .unwrap()
        .take();

    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "font" WHERE "image" LIKE '%'"#
    );
}

#[test]
fn insert_6() {
    let select = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("glyph")
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .column("id")
        .column("image")
        .column("aspect")
        .table_name("cte")
        .to_owned();
    let with_clause = WithClause::new().cte(cte).to_owned();
    let select = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("cte")
        .to_owned();
    let mut insert = Query::insert();
    insert
        .into_table("glyph")
        .columns(["id", "image", "aspect"])
        .select_from(select)
        .unwrap();
    let query = insert.with(with_clause);
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph")"#,
            r#"INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#,
        ].join(" ")
    );
}

#[test]
fn insert_7() {
    let query = Query::insert()
        .into_table("glyph")
        .or_default_values()
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"INSERT INTO "glyph" VALUES (DEFAULT)"#
    );
}

#[test]
fn insert_8() {
    let query = Query::insert()
        .into_table("glyph")
        .or_default_values()
        .returning_col("id")
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"INSERT INTO "glyph" VALUES (DEFAULT) RETURNING "id""#
    );
}

#[test]
fn insert_10() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "tokens"])
            .values_panic([
                f64::consts::PI.into(),
                vec![
                    "Token1".to_string(),
                    "Token2".to_string(),
                    "Token3".to_string()
                ]
                .into()
            ])
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("aspect", "tokens") VALUES (3.141592653589793, ARRAY ['Token1','Token2','Token3'])"#
    );
}

#[test]
fn insert_float_and_empty_array() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["aspect", "tokens"])
            .values_panic([f64::consts::PI.into(), Vec::<String>::new().into()])
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"INSERT INTO "glyph" ("aspect", "tokens") VALUES (3.141592653589793, '{}')"#
    );
}

#[test]
fn insert_11() {
    let select = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("glyph")
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .column("id")
        .column("image")
        .column("aspect")
        .table_name("cte")
        .to_owned();
    let select = SelectStatement::new()
        .columns(["id", "image", "aspect"])
        .from("cte")
        .to_owned();
    let insert = Query::insert()
        .into_table("glyph")
        .columns(["id", "image", "aspect"])
        .with_cte(cte)
        .select_from(select)
        .unwrap()
        .take();
    assert_eq!(
        insert.to_string(PostgresQueryBuilder).unwrap(),
        [
            r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph")"#,
            r#"INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#,
        ].join(" ")
    );
}
