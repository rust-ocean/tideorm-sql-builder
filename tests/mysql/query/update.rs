use super::*;

#[test]
fn update_1() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .values([
                ("aspect", 2.1345.into()),
                ("image", "24B0E11951B03B07F8300FD003983F03F0780060".into()),
            ])
            .and_where(Expr::col("id").eq(1))
            .order_by("id", Order::Asc)
            .limit(1)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "UPDATE `glyph` SET `aspect` = 2.1345, `image` = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE `id` = 1 ORDER BY `id` ASC LIMIT 1"
    );
}

#[test]
fn update_3() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("aspect", Expr::cust("60 * 24 * 24"))
            .values([("image", "24B0E11951B03B07F8300FD003983F03F0780060".into()),])
            .and_where(Expr::col("id").eq(1))
            .order_by("id", Order::Asc)
            .limit(1)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "UPDATE `glyph` SET `aspect` = 60 * 24 * 24, `image` = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE `id` = 1 ORDER BY `id` ASC LIMIT 1"
    );
}

#[test]
fn update_4() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("aspect", Expr::col("aspect").add(1))
            .values([("image", "24B0E11951B03B07F8300FD003983F03F0780060".into()),])
            .and_where(Expr::col("id").eq(1))
            .order_by("id", Order::Asc)
            .limit(1)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "UPDATE `glyph` SET `aspect` = `aspect` + 1, `image` = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE `id` = 1 ORDER BY `id` ASC LIMIT 1"
    );
}

#[test]
fn update_from_two_tables_joins_both() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("tokens", Expr::column(("character", "character")))
            .from("character")
            .from("font")
            .cond_where(
                Expr::col(("glyph", "image"))
                    .eq(Expr::col(("character", "user_data")))
                    .and(Expr::col(("character", "font_id")).eq(Expr::col(("font", "id")))),
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "UPDATE `glyph` JOIN `character` JOIN `font` ON `glyph`.`image` = `character`.`user_data` AND `character`.`font_id` = `font`.`id` SET `glyph`.`tokens` = `character`.`character`"
    );
}

#[test]
fn a_joined_update_is_ordered_and_limited_on_mariadb_only() {
    let update = Query::update()
        .table("glyph")
        .value("tokens", Expr::column(("character", "character")))
        .from("character")
        .cond_where(Expr::col(("glyph", "id")).eq(Expr::col(("character", "id"))))
        .order_by(("glyph", "id"), Order::Desc)
        .limit(1)
        .to_owned();
    assert_eq!(
        update.to_string(MariaDbDialect).unwrap(),
        "UPDATE `glyph` JOIN `character` ON `glyph`.`id` = `character`.`id` SET `glyph`.`tokens` = `character`.`character` ORDER BY `glyph`.`id` DESC LIMIT 1"
    );
    assert_eq!(
        update.to_string(MysqlQueryBuilder).unwrap_err(),
        RenderError::Unsupported(
            "MySQL has no ORDER BY or LIMIT in an UPDATE that joins other tables".into()
        )
    );
}

#[test]
fn update_from_names_an_aliased_or_qualified_target() {
    let update = |target: TableRef, qualifier: &str| {
        Query::update()
            .table(target)
            .value("tokens", Expr::column(("character", "character")))
            .from("character")
            .cond_where(Expr::col(("character", "id")).eq(1))
            .to_string(MysqlQueryBuilder)
            .unwrap()
            .ends_with(&format!(
                "SET {qualifier}.`tokens` = `character`.`character`"
            ))
    };
    assert!(update(
        ("library", "glyph").into_table_ref(),
        "`library`.`glyph`"
    ));
    assert!(update("glyph".into_table_ref().alias("g"), "`g`"));
}
