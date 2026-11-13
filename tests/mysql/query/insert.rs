use super::*;

#[test]
#[allow(clippy::approx_constant)]
fn insert_2() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image", "aspect",])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "INSERT INTO `glyph` (`image`, `aspect`) VALUES ('04108048005887010020060000204E0180400400', 3.1415)"
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_3() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image", "aspect",])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .values_panic([Value::String(None).into(), 2.1345.into(),])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "INSERT INTO `glyph` (`image`, `aspect`) VALUES ('04108048005887010020060000204E0180400400', 3.1415), (NULL, 2.1345)"
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
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "INSERT INTO `glyph` (`image`) VALUES ('1970-01-01 00:00:00.000000')"
    );
}

#[test]

fn insert_5() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .columns(["image"])
            .values_panic([uuid::Uuid::nil().into()])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "INSERT INTO `glyph` (`image`) VALUES ('00000000-0000-0000-0000-000000000000')"
    );
}

#[test]
fn insert_6() {
    assert_eq!(
        Query::insert()
            .into_table("glyph")
            .or_default_values()
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "INSERT INTO `glyph` VALUES ()"
    );
}

#[test]
fn returning_is_mariadb_only() {
    let insert = Query::insert()
        .into_table("glyph")
        .or_default_values()
        .returning_col("id")
        .to_owned();
    assert_eq!(
        insert.to_string(MariaDbDialect).unwrap(),
        "INSERT INTO `glyph` VALUES () RETURNING `id`"
    );
    let delete = Query::delete()
        .from_table("glyph")
        .and_where(Expr::col("id").eq(1))
        .returning_all()
        .to_owned();
    for error in [
        insert.to_string(MysqlQueryBuilder).unwrap_err(),
        delete.to_string(MysqlQueryBuilder).unwrap_err(),
    ] {
        assert_eq!(
            error,
            RenderError::Unsupported("MySQL has no RETURNING".into())
        );
    }
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
                            x.and_where(Expr::col("id").eq(6i32));
                        },
                    )
                    .to_owned()
            )
            .unwrap()
            .to_owned()
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        r"INSERT INTO `glyph` (`aspect`, `image`) SELECT `aspect`, `image` FROM `glyph` WHERE `image` LIKE '%'"
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_0() {
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
                    .update_columns(["aspect", "image"])
                    .to_owned()
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = VALUES(`image`)",
        ]
        .join(" ")
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
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`)",
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
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = VALUES(`image`)",
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
            .to_string(MysqlQueryBuilder).unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `aspect` = '04108048005887010020060000204E0180400400', `image` = 3.1415",
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
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `image` = 1 + 2",
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
            .to_string(MysqlQueryBuilder).unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `aspect` = '04108048005887010020060000204E0180400400', `image` = VALUES(`image`)",
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
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('04108048005887010020060000204E0180400400', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = 1 + 2",
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
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            r"INSERT INTO `glyph` (`aspect`, `image`)",
            r"VALUES ('abcd', 3.1415)",
            r"ON DUPLICATE KEY UPDATE `id` = `id`",
        ]
        .join(" ")
    );
}
