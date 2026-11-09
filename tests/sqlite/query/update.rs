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
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    );
}

#[test]
fn update_3() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("aspect", Expr::cust("60 * 24 * 24"))
            .values([("image", "24B0E11951B03B07F8300FD003983F03F0780060".into())])
            .and_where(Expr::col("id").eq(1))
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    );
}

#[test]
fn update_4() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("aspect", Expr::col("aspect").add(1))
            .values([("image", "24B0E11951B03B07F8300FD003983F03F0780060".into())])
            .and_where(Expr::col("id").eq(1))
            .order_by("id", Order::Asc)
            .limit(1)
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"UPDATE "glyph" SET "aspect" = "aspect" + 1, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 ORDER BY "id" ASC LIMIT 1"#
    );
}

#[test]
fn update_returning_all_columns() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("aspect", Expr::cust("60 * 24 * 24"))
            .values([("image", "24B0E11951B03B07F8300FD003983F03F0780060".into())])
            .and_where(Expr::col("id").eq(1))
            .returning(Query::returning().all())
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 RETURNING *"#
    );
}

#[test]
fn update_returning_specified_columns() {
    assert_eq!(
        Query::update()
            .table("glyph")
            .value("aspect", Expr::cust("60 * 24 * 24"))
            .values([("image", "24B0E11951B03B07F8300FD003983F03F0780060".into())])
            .and_where(Expr::col("id").eq(1))
            .returning(Query::returning().columns(["id", "image"]))
            .to_string(SqliteQueryBuilder)
            .unwrap(),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 RETURNING "id", "image""#
    );
}
