use super::*;

#[test]
fn delete_1() {
    assert_eq!(
        Query::delete()
            .from_table("glyph")
            .and_where(Expr::col("id").eq(1))
            .order_by("id", Order::Asc)
            .limit(1)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "DELETE FROM `glyph` WHERE `id` = 1 ORDER BY `id` ASC LIMIT 1"
    );
}
