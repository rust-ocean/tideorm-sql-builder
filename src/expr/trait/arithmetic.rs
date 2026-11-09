use super::*;

pub(super) fn add<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Add, right)
}

pub(super) fn div<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Div, right)
}

pub(super) fn left_shift<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::LShift, right)
}

pub(super) fn modulo<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Mod, right)
}

pub(super) fn mul<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Mul, right)
}

pub(super) fn right_shift<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::RShift, right)
}

pub(super) fn sub<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Sub, right)
}

pub(super) fn bit_and<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::BitAnd, right)
}

pub(super) fn bit_or<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::BitOr, right)
}
