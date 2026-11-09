//! Base types used throughout tideorm-sql-builder.

use crate::{Values, expr::*, query::*};
use std::fmt::Debug;

use crate::extension::postgres::PgBinOper;

use crate::extension::sqlite::SqliteBinOper;

// Intentionally not `pub`, so that we're free to experiment with the internal structure.
mod iden;

pub use iden::*;

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOper {
    Not,
}

/// Binary operators.
///
/// If something is not supported here, you can use [`BinOper::Custom`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOper {
    And,
    Or,
    Like,
    NotLike,
    Is,
    IsNot,
    In,
    NotIn,
    Between,
    NotBetween,
    Equal,
    NotEqual,
    SmallerThan,
    GreaterThan,
    SmallerThanOrEqual,
    GreaterThanOrEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    LShift,
    RShift,
    As,
    Escape,
    Custom(&'static str),

    PgOperator(PgBinOper),

    SqliteOperator(SqliteBinOper),
}

/// Logical chain operator: conjunction or disjunction.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalChainOper {
    And(Expr),
    Or(Expr),
}

/// Join types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Join,
    CrossJoin,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    StraightJoin,
}

/// Nulls order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullOrdering {
    First,
    Last,
}

/// Order expression
#[derive(Debug, Clone, PartialEq)]
pub struct OrderExpr {
    pub(crate) expr: Expr,
    pub(crate) order: Order,
    pub(crate) nulls: Option<NullOrdering>,
}

/// Join on types
#[derive(Debug, Clone, PartialEq)]
pub enum JoinOn {
    Condition(Box<ConditionHolder>),
    Columns(Vec<Expr>),
}

/// Ordering options
#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Asc,
    Desc,
    Field(Values),
}

/// Known SQL keywords that can be used as expressions.
///
/// If something is not supported here, you can use [`Keyword::Custom`].
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Null,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Default,
    Custom(DynIden),
}

/// Like Expression
#[derive(Debug, Clone)]
pub struct LikeExpr {
    pub(crate) pattern: String,
    pub(crate) escape: Option<char>,
}

impl LikeExpr {
    pub fn new<T>(pattern: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            escape: None,
        }
    }

    pub fn escape(self, c: char) -> Self {
        Self {
            pattern: self.pattern,
            escape: Some(c),
        }
    }
}

pub trait IntoLikeExpr: Into<LikeExpr> {
    fn into_like_expr(self) -> LikeExpr;
}

impl<T> IntoLikeExpr for T
where
    T: Into<LikeExpr>,
{
    fn into_like_expr(self) -> LikeExpr {
        self.into()
    }
}

impl<T> From<T> for LikeExpr
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        LikeExpr::new(value)
    }
}

/// SubQuery operators
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SubQueryOper {
    Exists,
    Any,
    Some,
    All,
}
