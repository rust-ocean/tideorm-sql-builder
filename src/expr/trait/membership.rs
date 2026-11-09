use super::*;

pub(super) fn in_subquery<T: ExprTrait>(this: T, sel: SelectStatement) -> Expr {
    this.binary(BinOper::In, Expr::SubQuery(None, Box::new(sel.into())))
}

pub(super) fn in_tuples<T: ExprTrait, V, I>(this: T, v: I) -> Expr
where
    V: IntoValueTuple,
    I: IntoIterator<Item = V>,
{
    this.binary(
        BinOper::In,
        Expr::Tuple(
            v.into_iter()
                .map(|m| Expr::Values(m.into_value_tuple().into_iter().collect()))
                .collect(),
        ),
    )
}

pub(super) fn is_in<T: ExprTrait, V, I>(this: T, v: I) -> Expr
where
    V: Into<Expr>,
    I: IntoIterator<Item = V>,
{
    this.binary(
        BinOper::In,
        Expr::Tuple(v.into_iter().map(|v| v.into()).collect()),
    )
}

pub(super) fn is_not_in<T: ExprTrait, V, I>(this: T, v: I) -> Expr
where
    V: Into<Expr>,
    I: IntoIterator<Item = V>,
{
    this.binary(
        BinOper::NotIn,
        Expr::Tuple(v.into_iter().map(|v| v.into()).collect()),
    )
}

pub(super) fn not_in_subquery<T: ExprTrait>(this: T, sel: SelectStatement) -> Expr {
    this.binary(BinOper::NotIn, Expr::SubQuery(None, Box::new(sel.into())))
}
