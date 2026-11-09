use crate::*;

#[test]
fn test_blank_condition() {
    let query = Query::select()
        .column("image")
        .from("glyph")
        .cond_where(Cond::all())
        .cond_where(Expr::val(1).eq(1))
        .cond_where(Expr::val(2).eq(2))
        .cond_where(Cond::any().add(Expr::val(3).eq(3)).add(Expr::val(4).eq(4)))
        .to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder).unwrap(),
        "SELECT `image` FROM `glyph` WHERE 1 = 1 AND 2 = 2 AND (3 = 3 OR 4 = 4)"
    );
}
