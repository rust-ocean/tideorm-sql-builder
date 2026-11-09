use crate::*;

fn id() -> Expr {
    Expr::col("id")
}

#[test]
fn constant_true_forms_fold_to_true() {
    let empty: Vec<i32> = Vec::new();
    for condition in [
        Cond::all(),
        Cond::all().add(Expr::val(true)),
        Cond::all().add(Expr::cust("TRUE")),
        Cond::all().add(Expr::cust("(1 = 1)")),
        Cond::all().add(Expr::cust("true = true")),
        Cond::all().add(id().is_not_in(empty)),
        Cond::all().add(Expr::val(1).eq(1)),
        Cond::all().add(Expr::val("a").ne("b")),
        Cond::all().add(Expr::val(false).not()),
        Cond::all().add(Expr::val(false).or(Expr::val(true))),
        Cond::any().add(id().eq(1)).add(Expr::cust("1")),
        Cond::all().add(Cond::any().add(Expr::val(false)).not()),
        Cond::all()
            .add(Cond::all().add(Expr::cust("1=1")))
            .add(Expr::val(true)),
    ] {
        assert!(condition.is_trivially_true(), "{condition:?}");
    }
}

#[test]
fn a_condition_that_can_exclude_a_row_is_not_trivially_true() {
    let empty: Vec<i32> = Vec::new();
    for condition in [
        Cond::any(),
        Cond::all().add(id().eq(1)),
        Cond::all().add(Expr::val(true)).add(id().eq(1)),
        Cond::all().add(id().is_in(empty.clone())),
        Cond::any().add(id().is_in(empty)).add(Expr::val(false)),
        Cond::all().add(Expr::cust("1 = 1 AND id = 2")),
        Cond::all().add(Expr::cust("TRUE -- comment")),
        Cond::all().add(Expr::val(true)).not(),
        // NULL compares unknown, and values of two types as the database does.
        Cond::all().add(Expr::val(Option::<i32>::None).eq(Option::<i32>::None)),
        Cond::all().add(Expr::val(1).eq(1.0)),
        Cond::all().add(id().eq(id())),
    ] {
        assert!(!condition.is_trivially_true(), "{condition:?}");
    }
}
