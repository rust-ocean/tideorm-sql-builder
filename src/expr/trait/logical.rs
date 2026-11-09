use super::*;

pub(super) fn and<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::And, right)
}

pub(super) fn not<T: ExprTrait>(this: T) -> Expr {
    this.unary(UnOper::Not)
}

pub(super) fn or<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Or, right)
}
