use crate::{Expr, func::*, query::*, types::*, value::*};

mod arithmetic;
mod comparison;
mod logical;
mod membership;
mod patterns;

/// "Operator" methods for building expressions.
///
/// They can be called directly on any expression type, without constructing an
/// [`Expr`] first:
///
/// ```no_run
/// # use sql_builder::*;
/// #
/// let expr = 1_i32.cast_as("REAL");
/// let expr = Func::char_length("abc").eq(3_i32);
/// let expr = Expr::current_date().cast_as("TEXT").like("2024%");
/// ```
///
/// If some methods are missing, look into [`BinOper::Custom`], [`Func::cust`],
/// or [`Expr::cust*`][`Expr::cust_with_values`].
pub trait ExprTrait: Sized {
    #[doc = include_str!("trait/docs/add.md")]
    fn add<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::add(self, right)
    }

    #[doc = include_str!("trait/docs/as_enum.md")]
    #[allow(clippy::wrong_self_convention)]
    fn as_enum<N>(self, type_name: N) -> Expr
    where
        N: IntoIden;

    #[doc = include_str!("trait/docs/and.md")]
    fn and<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        logical::and(self, right)
    }

    #[doc = include_str!("trait/docs/between.md")]
    fn between<A, B>(self, a: A, b: B) -> Expr
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        comparison::between(self, a, b)
    }

    #[doc = include_str!("trait/docs/binary.md")]
    fn binary<O, R>(self, op: O, right: R) -> Expr
    where
        O: Into<BinOper>,
        R: Into<Expr>;

    #[doc = include_str!("trait/docs/cast_as.md")]
    fn cast_as<N>(self, type_name: N) -> Expr
    where
        N: IntoIden;

    #[doc = include_str!("trait/docs/div.md")]
    fn div<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::div(self, right)
    }

    #[doc = include_str!("trait/docs/eq.md")]
    fn eq<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::eq(self, right)
    }

    #[doc = include_str!("trait/docs/equals.md")]
    fn equals<C>(self, col: C) -> Expr
    where
        C: IntoColumnRef,
    {
        comparison::equals(self, col)
    }

    #[doc = include_str!("trait/docs/gt.md")]
    fn gt<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::gt(self, right)
    }

    #[doc = include_str!("trait/docs/gte.md")]
    fn gte<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::gte(self, right)
    }

    #[doc = include_str!("trait/docs/in_subquery.md")]
    fn in_subquery(self, sel: SelectStatement) -> Expr {
        membership::in_subquery(self, sel)
    }

    #[doc = include_str!("trait/docs/in_tuples.md")]
    fn in_tuples<V, I>(self, v: I) -> Expr
    where
        V: IntoValueTuple,
        I: IntoIterator<Item = V>,
    {
        membership::in_tuples(self, v)
    }

    #[doc = include_str!("trait/docs/is.md")]
    fn is<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::is(self, right)
    }

    #[doc = include_str!("trait/docs/is_in.md")]
    #[allow(clippy::wrong_self_convention)]
    fn is_in<V, I>(self, v: I) -> Expr
    where
        V: Into<Expr>,
        I: IntoIterator<Item = V>,
    {
        membership::is_in(self, v)
    }

    #[doc = include_str!("trait/docs/is_not.md")]
    #[allow(clippy::wrong_self_convention)]
    fn is_not<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::is_not(self, right)
    }

    #[doc = include_str!("trait/docs/is_not_in.md")]
    #[allow(clippy::wrong_self_convention)]
    fn is_not_in<V, I>(self, v: I) -> Expr
    where
        V: Into<Expr>,
        I: IntoIterator<Item = V>,
    {
        membership::is_not_in(self, v)
    }

    #[doc = include_str!("trait/docs/is_not_null.md")]
    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> Expr {
        comparison::is_not_null(self)
    }

    #[doc = include_str!("trait/docs/is_null.md")]
    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> Expr {
        comparison::is_null(self)
    }

    #[doc = include_str!("trait/docs/left_shift.md")]
    fn left_shift<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::left_shift(self, right)
    }

    #[doc = include_str!("trait/docs/like.md")]
    fn like<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        patterns::like(self, like)
    }

    #[doc = include_str!("trait/docs/lt.md")]
    fn lt<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::lt(self, right)
    }

    #[doc = include_str!("trait/docs/lte.md")]
    fn lte<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::lte(self, right)
    }

    #[doc = include_str!("trait/docs/modulo.md")]
    fn modulo<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::modulo(self, right)
    }

    #[doc = include_str!("trait/docs/mul.md")]
    fn mul<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::mul(self, right)
    }

    #[doc = include_str!("trait/docs/ne.md")]
    fn ne<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        comparison::ne(self, right)
    }

    #[doc = include_str!("trait/docs/not.md")]
    fn not(self) -> Expr {
        logical::not(self)
    }

    #[doc = include_str!("trait/docs/not_between.md")]
    fn not_between<A, B>(self, a: A, b: B) -> Expr
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        comparison::not_between(self, a, b)
    }

    #[doc = include_str!("trait/docs/not_equals.md")]
    fn not_equals<C>(self, col: C) -> Expr
    where
        C: IntoColumnRef,
    {
        comparison::not_equals(self, col)
    }

    #[doc = include_str!("trait/docs/not_in_subquery.md")]
    fn not_in_subquery(self, sel: SelectStatement) -> Expr {
        membership::not_in_subquery(self, sel)
    }

    #[doc = include_str!("trait/docs/not_like.md")]
    fn not_like<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        patterns::not_like(self, like)
    }

    #[doc = include_str!("trait/docs/or.md")]
    fn or<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        logical::or(self, right)
    }

    #[doc = include_str!("trait/docs/right_shift.md")]
    fn right_shift<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::right_shift(self, right)
    }

    #[doc = include_str!("trait/docs/sub.md")]
    fn sub<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::sub(self, right)
    }

    #[doc = include_str!("trait/docs/unary.md")]
    fn unary(self, o: UnOper) -> Expr;

    #[doc = include_str!("trait/docs/bit_and.md")]
    fn bit_and<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::bit_and(self, right)
    }

    #[doc = include_str!("trait/docs/bit_or.md")]
    fn bit_or<R>(self, right: R) -> Expr
    where
        R: Into<Expr>,
    {
        arithmetic::bit_or(self, right)
    }

    #[doc = include_str!("trait/docs/max.md")]
    fn max(self) -> Expr;

    #[doc = include_str!("trait/docs/min.md")]
    fn min(self) -> Expr;

    #[doc = include_str!("trait/docs/sum.md")]
    fn sum(self) -> Expr;

    #[doc = include_str!("trait/docs/avg.md")]
    fn avg(self) -> Expr;

    #[doc = include_str!("trait/docs/count.md")]
    fn count(self) -> Expr;

    #[doc = include_str!("trait/docs/count_distinct.md")]
    fn count_distinct(self) -> Expr;

    #[doc = include_str!("trait/docs/if_null.md")]
    fn if_null<V>(self, v: V) -> Expr
    where
        V: Into<Expr>;
}

/// This generic implementation covers all expression types,
/// including [ColumnRef], [Value], [FunctionCall], [Expr]...
impl<T> ExprTrait for T
where
    T: Into<Expr>,
{
    fn as_enum<N>(self, type_name: N) -> Expr
    where
        N: IntoIden,
    {
        Expr::AsEnum(type_name.into_iden(), Box::new(self.into()))
    }

    fn binary<O, R>(self, op: O, right: R) -> Expr
    where
        O: Into<BinOper>,
        R: Into<Expr>,
    {
        Expr::Binary(Box::new(self.into()), op.into(), Box::new(right.into()))
    }

    fn cast_as<N>(self, type_name: N) -> Expr
    where
        N: IntoIden,
    {
        Expr::FunctionCall(Func::cast_as(self, type_name))
    }

    fn unary(self, op: UnOper) -> Expr {
        Expr::Unary(op, Box::new(self.into()))
    }

    fn max(self) -> Expr {
        Func::max(self).into()
    }

    fn min(self) -> Expr {
        Func::min(self).into()
    }

    fn sum(self) -> Expr {
        Func::sum(self).into()
    }

    fn avg(self) -> Expr {
        Func::avg(self).into()
    }

    fn count(self) -> Expr {
        Func::count(self).into()
    }

    fn count_distinct(self) -> Expr {
        Func::count_distinct(self).into()
    }

    fn if_null<V>(self, v: V) -> Expr
    where
        V: Into<Expr>,
    {
        Func::if_null(self, v).into()
    }
}
