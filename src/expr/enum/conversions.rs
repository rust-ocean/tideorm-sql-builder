//! What converts into an [`Expr`]: any value, the parts an expression is built from, and
//! whole statements, which become subqueries.

use super::*;

/// Anything that is a [`Value`] is a bound value.
impl<T> From<T> for Expr
where
    T: Into<Value>,
{
    fn from(v: T) -> Self {
        Self::Value(v.into())
    }
}

/// Each type is the payload of one `Expr` variant.
macro_rules! into_variant {
    ($($ty:ty => $variant:ident),+ $(,)?) => {$(
        impl From<$ty> for Expr {
            fn from(v: $ty) -> Self {
                Self::$variant(v)
            }
        }
    )+};
}

into_variant! {
    Vec<Value> => Values,
    FunctionCall => FunctionCall,
    ColumnRef => Column,
    Keyword => Keyword,
    TypeRef => TypeName,
}

/// A statement used as an expression is an unnamed subquery.
impl From<SubQueryStatement> for Expr {
    fn from(v: SubQueryStatement) -> Self {
        Self::SubQuery(None, Box::new(v))
    }
}

macro_rules! into_subquery {
    ($($ty:ty),+ $(,)?) => {$(
        impl From<$ty> for Expr {
            fn from(v: $ty) -> Self {
                SubQueryStatement::from(v).into()
            }
        }
    )+};
}

into_subquery!(
    SelectStatement,
    InsertStatement,
    UpdateStatement,
    DeleteStatement,
    WithQuery,
);

/// The pattern, followed by `ESCAPE '<char>'` when it names an escape character.
impl From<LikeExpr> for Expr {
    fn from(like: LikeExpr) -> Self {
        let pattern = like.pattern.into();
        match like.escape {
            Some(escape) => Self::Binary(
                Box::new(pattern),
                BinOper::Escape,
                Box::new(Self::Constant(escape.into())),
            ),
            None => pattern,
        }
    }
}
