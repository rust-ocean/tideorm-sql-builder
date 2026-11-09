use super::*;

pub(super) fn like<T: ExprTrait, L>(this: T, like: L) -> Expr
where
    L: IntoLikeExpr,
{
    this.binary(BinOper::Like, like.into_like_expr())
}

pub(super) fn not_like<T: ExprTrait, L>(this: T, like: L) -> Expr
where
    L: IntoLikeExpr,
{
    this.binary(BinOper::NotLike, like.into_like_expr())
}
