#[test]
fn select_62() {
    assert_eq!(
        Query::select()
            .from("character")
            .expr(
                Expr::col("character")
                    .max()
                    .over(WindowStatement::partition_by("font_size"))
                    .alias("C"),
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        r#"SELECT MAX(`character`) OVER ( PARTITION BY `font_size` ) AS `C` FROM `character`"#
    );
}

use super::*;

#[test]
fn select_63() {
    assert_eq!(
        Query::select()
            .from("character")
            .expr(Expr::col("character").max().over("w"))
            .window("w", WindowStatement::partition_by("font_size"))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        r#"SELECT MAX(`character`) OVER `w` FROM `character` WINDOW `w` AS (PARTITION BY `font_size`)"#
    );
}

#[test]
fn md5_fn() {
    assert_eq!(
        Query::select()
            .expr(Func::md5(Expr::val("test")))
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        r#"SELECT MD5('test')"#
    );
}
