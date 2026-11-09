use super::*;

#[test]
fn delete_1() {
    let query = Query::delete()
        .from_table("glyph")
        .and_where(Expr::col("id").eq(1))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder).unwrap(),
        r#"DELETE FROM "glyph" WHERE "id" = 1"#
    );
}

#[test]
fn delete_returning_all_columns() {
    assert_eq!(
        Query::delete()
            .from_table("glyph")
            .and_where(Expr::col("id").eq(1))
            .returning(Query::returning().all())
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    );
}

#[test]
fn delete_returning_specific_columns() {
    assert_eq!(
        Query::delete()
            .from_table("glyph")
            .and_where(Expr::col("id").eq(1))
            .returning(Query::returning().columns(["id", "image"]))
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id", "image""#
    );
}

#[test]
fn delete_returning_specific_exprs() {
    assert_eq!(
        Query::delete()
            .from_table("glyph")
            .and_where(Expr::col("id").eq(1))
            .returning(Query::returning().exprs([Expr::col("id"), Expr::col("image")]))
            .to_string(PostgresQueryBuilder)
            .unwrap(),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id", "image""#
    );
}
