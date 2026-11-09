use super::*;

#[test]
fn select_1() {
    assert_eq!(
        Query::select()
            .columns(["character", "size_w", "size_h"])
            .from("character")
            .limit(10)
            .offset(100)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character`, `size_w`, `size_h` FROM `character` LIMIT 10 OFFSET 100"
    );
}

#[test]
fn select_2() {
    assert_eq!(
        Query::select()
            .columns(["character", "size_w", "size_h"])
            .from("character")
            .and_where(Expr::col("size_w").eq(3))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 3"
    );
}

#[test]
fn select_3() {
    assert_eq!(
        Query::select()
            .columns(["character", "size_w", "size_h"])
            .from("character")
            .and_where(Expr::col("size_w").eq(3))
            .and_where(Expr::col("size_h").eq(4))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 3 AND `size_h` = 4"
    );
}

#[test]
fn select_4() {
    assert_eq!(
        Query::select()
            .columns(["image"])
            .from_subquery(
                Query::select()
                    .columns(["image", "aspect"])
                    .from("glyph")
                    .take(),
                "subglyph"
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `image` FROM (SELECT `image`, `aspect` FROM `glyph`) AS `subglyph`"
    );
}

#[test]
fn select_5() {
    assert_eq!(
        Query::select()
            .column(("glyph", "image"))
            .from("glyph")
            .and_where(Expr::col(("glyph", "aspect")).is_in([3, 4]))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `glyph`.`image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4)"
    );
}

#[test]
fn select_6() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .exprs([Expr::col("image").max()])
            .from("glyph")
            .group_by_columns(["aspect"])
            .and_having(Expr::col("aspect").gt(2))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` > 2"
    );
}

#[test]
fn select_7() {
    assert_eq!(
        Query::select()
            .columns(["aspect"])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2"
    );
}

#[test]
fn select_8() {
    assert_eq!(
        Query::select()
            .columns(["character",])
            .from("character")
            .left_join(
                "font",
                Expr::col(("character", "font_id")).equals(("font", "id"))
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id`"
    );
}

#[test]
fn select_9() {
    assert_eq!(
        Query::select()
            .columns(["character",])
            .from("character")
            .left_join(
                "font",
                Expr::col(("character", "font_id")).equals(("font", "id"))
            )
            .inner_join(
                "glyph",
                Expr::col(("character", "character")).equals(("glyph", "image"))
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` INNER JOIN `glyph` ON `character`.`character` = `glyph`.`image`"
    );
}

#[test]
fn select_10() {
    assert_eq!(
        Query::select()
            .columns(["character",])
            .from("character")
            .left_join(
                "font",
                Expr::col(("character", "font_id"))
                    .equals(("font", "id"))
                    .and(Expr::col(("character", "font_id")).equals(("font", "id")))
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"
    );
}

#[test]
fn select_11() {
    assert_eq!(
        Query::select()
            .columns(["aspect",])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by("image", Order::Desc)
            .order_by(("glyph", "aspect"), Order::Asc)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"
    );
}

#[test]
fn select_12() {
    assert_eq!(
        Query::select()
            .columns(["aspect",])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by_columns([("id", Order::Asc), ("aspect", Order::Desc),])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `id` ASC, `aspect` DESC"
    );
}

#[test]
fn select_13() {
    assert_eq!(
        Query::select()
            .columns(["aspect",])
            .from("glyph")
            .and_where(Expr::col("aspect").if_null(0).gt(2))
            .order_by_columns([
                (("glyph", "id"), Order::Asc),
                (("glyph", "aspect"), Order::Desc),
            ])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `glyph`.`id` ASC, `glyph`.`aspect` DESC"
    );
}

#[test]
fn select_14() {
    assert_eq!(
        Query::select()
            .columns(["id", "aspect",])
            .expr(Expr::col("image").max())
            .from("glyph")
            .group_by_columns([("glyph", "id"), ("glyph", "aspect"),])
            .and_having(Expr::col("aspect").gt(2))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `id`, `aspect`, MAX(`image`) FROM `glyph` GROUP BY `glyph`.`id`, `glyph`.`aspect` HAVING `aspect` > 2"
    );
}

#[test]
fn select_15() {
    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(Expr::col("font_id").is_null())
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `font_id` IS NULL"
    );
}

#[test]
fn select_16() {
    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(Expr::col("font_id").is_null())
            .and_where(Expr::col("character").is_not_null())
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `font_id` IS NULL AND `character` IS NOT NULL"
    );
}

#[test]
fn select_17() {
    assert_eq!(
        Query::select()
            .columns([("glyph", "image")])
            .from("glyph")
            .and_where(Expr::col(("glyph", "aspect")).between(3, 5))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `glyph`.`image` FROM `glyph` WHERE `glyph`.`aspect` BETWEEN 3 AND 5"
    );
}

#[test]
fn select_18() {
    assert_eq!(
        Query::select()
            .columns(["aspect",])
            .from("glyph")
            .and_where(Expr::col("aspect").between(3, 5))
            .and_where(Expr::col("aspect").not_between(8, 10))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `aspect` FROM `glyph` WHERE (`aspect` BETWEEN 3 AND 5) AND (`aspect` NOT BETWEEN 8 AND 10)"
    );
}

#[test]
fn select_19() {
    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .and_where(Expr::col("character").eq("A"))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `character` = 'A'"
    );
}

#[test]
fn select_20() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .and_where(Expr::col("character").like("A"))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `character` LIKE 'A'"
    );
}

#[test]
fn select_21() {
    assert_eq!(
        Query::select()
            .columns(["character"])
            .from("character")
            .cond_where(any![
                Expr::col("character").like("A%"),
                Expr::col("character").like("%B"),
                Expr::col("character").like("%C%"),
            ])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `character` LIKE 'A%' OR `character` LIKE '%B' OR `character` LIKE '%C%'"
    );
}

#[test]
fn select_22() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .cond_where(
                Cond::all()
                    .add(
                        Cond::any().add(Expr::col("character").like("C")).add(
                            Expr::col("character")
                                .like("D")
                                .and(Expr::col("character").like("E"))
                        )
                    )
                    .add(
                        Expr::col("character")
                            .like("F")
                            .or(Expr::col("character").like("G"))
                    )
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE (`character` LIKE 'C' OR (`character` LIKE 'D' AND `character` LIKE 'E')) AND (`character` LIKE 'F' OR `character` LIKE 'G')"
    );
}

#[test]
fn select_23() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .and_where_option(None)
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character`"
    );
}

#[test]
fn select_24() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col("font_id").eq(5));
                },
                |_| ()
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `font_id` = 5"
    );
}

#[test]
fn select_25() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .and_where(Expr::col("size_w").mul(2).eq(Expr::col("size_h").div(2)))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE `size_w` * 2 = `size_h` / 2"
    );
}

#[test]
fn select_26() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .and_where(
                Expr::col("size_w")
                    .add(1)
                    .mul(2)
                    .eq(Expr::col("size_h").div(2).sub(1))
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` FROM `character` WHERE (`size_w` + 1) * 2 = (`size_h` / 2) - 1"
    );
}

#[test]
fn select_27() {
    assert_eq!(
        Query::select()
            .columns(["character", "size_w", "size_h"])
            .from("character")
            .and_where(Expr::col("size_w").eq(3))
            .and_where(Expr::col("size_h").eq(4))
            .and_where(Expr::col("size_h").eq(5))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 3 AND `size_h` = 4 AND `size_h` = 5"
    );
}

#[test]
fn select_28() {
    assert_eq!(
        Query::select()
            .columns(["character", "size_w", "size_h"])
            .from("character")
            .cond_where(any![
                Expr::col("size_w").eq(3),
                Expr::col("size_h").eq(4),
                Expr::col("size_h").eq(5),
            ])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 3 OR `size_h` = 4 OR `size_h` = 5"
    );
}

#[test]
fn select_30() {
    assert_eq!(
        Query::select()
            .columns(["character", "size_w", "size_h"])
            .from("character")
            .and_where(
                Expr::col("size_w")
                    .mul(2)
                    .add(Expr::col("size_h").div(3))
                    .eq(4)
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w` * 2) + (`size_h` / 3) = 4"
    );
}

#[test]
fn select_31() {
    assert_eq!(
        Query::select()
            .expr((1..10_i32).fold(Expr::value(0), |expr, i| { expr.add(i) }))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9"
    );
}

#[test]
fn select_32() {
    assert_eq!(
        Query::select()
            .expr_as(Expr::col("character"), "C")
            .from("character")
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` AS `C` FROM `character`"
    );

    // Same SQL as `expr_as`, but expressed via `SelectExprTrait`.
    assert_eq!(
        Query::select()
            .expr(Expr::col("character").alias("C"))
            .from("character")
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `character` AS `C` FROM `character`"
    );
}

#[test]
fn select_33() {
    assert_eq!(
        Query::select()
            .column("image")
            .from("glyph")
            .and_where(
                Expr::col("aspect")
                    .in_subquery(Query::select().expr(Expr::cust("3 + 2 * 2")).take())
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        "SELECT `image` FROM `glyph` WHERE `aspect` IN (SELECT 3 + 2 * 2)"
    );
}

#[test]
fn select_34a() {
    assert_eq!(
        Query::select()
            .column("aspect")
            .expr(Expr::col("image").max())
            .from("glyph")
            .group_by_columns(["aspect"])
            .cond_having(any![
                Expr::col("aspect").gt(2).or(Expr::col("aspect").lt(8)),
                Expr::col("aspect").gt(12).and(Expr::col("aspect").lt(18)),
                Expr::col("aspect").gt(32),
            ])
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            "SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect`",
            "HAVING `aspect` > 2 OR `aspect` < 8",
            "OR (`aspect` > 12 AND `aspect` < 18)",
            "OR `aspect` > 32",
        ]
        .join(" ")
    );
}

#[test]
fn select_35() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .and_where(Expr::col("aspect").is_null())
        .build(MysqlQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r"SELECT `id` FROM `glyph` WHERE `aspect` IS NULL"
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_36() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(Cond::any().add(Expr::col("aspect").is_null()))
        .build(MysqlQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r"SELECT `id` FROM `glyph` WHERE `aspect` IS NULL"
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_37() {
    let (statement, values) = Query::select()
        .column("id")
        .from("glyph")
        .cond_where(Cond::any().add(Cond::all()).add(Cond::any()))
        .build(MysqlQueryBuilder)
        .unwrap();

    assert_eq!(statement, r"SELECT `id` FROM `glyph` WHERE TRUE OR FALSE");
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
        .build(MysqlQueryBuilder)
        .unwrap();

    assert_eq!(
        statement,
        r"SELECT `id` FROM `glyph` WHERE NOT ((NOT TRUE) AND (NOT FALSE))"
    );
    assert_eq!(values.0, vec![]);
}
