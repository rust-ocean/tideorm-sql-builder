use super::*;

pub(super) fn between<T: ExprTrait, A, B>(this: T, a: A, b: B) -> Expr
where
    A: Into<Expr>,
    B: Into<Expr>,
{
    this.binary(
        BinOper::Between,
        Expr::Binary(Box::new(a.into()), BinOper::And, Box::new(b.into())),
    )
}

pub(super) fn eq<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Equal, right)
}

pub(super) fn equals<T: ExprTrait, C>(this: T, col: C) -> Expr
where
    C: IntoColumnRef,
{
    this.binary(BinOper::Equal, col.into_column_ref())
}

pub(super) fn gt<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::GreaterThan, right)
}

pub(super) fn gte<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::GreaterThanOrEqual, right)
}

pub(super) fn is<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::Is, right)
}

pub(super) fn is_not<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::IsNot, right)
}

pub(super) fn is_not_null<T: ExprTrait>(this: T) -> Expr {
    this.binary(BinOper::IsNot, Keyword::Null)
}

pub(super) fn is_null<T: ExprTrait>(this: T) -> Expr {
    this.binary(BinOper::Is, Keyword::Null)
}

pub(super) fn lt<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::SmallerThan, right)
}

pub(super) fn lte<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::SmallerThanOrEqual, right)
}

pub(super) fn ne<T: ExprTrait, R>(this: T, right: R) -> Expr
where
    R: Into<Expr>,
{
    this.binary(BinOper::NotEqual, right)
}

pub(super) fn not_between<T: ExprTrait, A, B>(this: T, a: A, b: B) -> Expr
where
    A: Into<Expr>,
    B: Into<Expr>,
{
    this.binary(
        BinOper::NotBetween,
        Expr::Binary(Box::new(a.into()), BinOper::And, Box::new(b.into())),
    )
}

pub(super) fn not_equals<T: ExprTrait, C>(this: T, col: C) -> Expr
where
    C: IntoColumnRef,
{
    this.binary(BinOper::NotEqual, col.into_column_ref())
}
