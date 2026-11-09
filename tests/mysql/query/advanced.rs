use super::*;

#[test]
fn union_1() {
    assert_eq!(
        Query::select()
            .column("character")
            .from("character")
            .union(
                UnionType::Distinct,
                Query::select()
                    .column("character")
                    .from("character")
                    .left_join(
                        "font",
                        Expr::col(("character", "font_id")).equals(("font", "id"))
                    )
                    .order_by(("font", "id"), Order::Asc)
                    .take()
            )
            .to_string(MysqlQueryBuilder)
            .unwrap(),
        [
            "SELECT `character` FROM `character` UNION (SELECT `character` FROM `character`",
            "LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` ORDER BY `font`.`id` ASC)"
        ]
        .join(" ")
    );
}

#[test]
fn sub_query_with_fn() {
    #[derive(Iden)]
    #[iden = "ARRAY"]
    pub struct ArrayFunc;

    let sub_select = Query::select()
        .column(Asterisk)
        .from("character")
        .to_owned();

    let select = Query::select()
        .expr(Func::cust(ArrayFunc).arg(Expr::SubQuery(None, Box::new(sub_select.into()))))
        .to_owned();

    assert_eq!(
        select.to_string(MysqlQueryBuilder).unwrap(),
        "SELECT ARRAY((SELECT * FROM `character`))"
    );
}
