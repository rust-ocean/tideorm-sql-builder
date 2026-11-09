//! For calling built-in SQL functions.

use crate::{expr::*, query::Condition, types::*};

pub use crate::extension::postgres::PgFunc;

mod aggregate;
mod bitwise;
mod call;
mod cast;

pub use call::*;

/// Known SQL functions.
///
/// If something is not supported here, you can use [`Func::Custom`].
#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Max,
    Min,
    Sum,
    Avg,
    Abs,
    Count,
    IfNull,
    Greatest,
    Least,
    CharLength,
    Cast,
    Custom(DynIden),
    Coalesce,
    Lower,
    Upper,
    BitAnd,
    BitOr,
    Random,
    Round,
    Md5,

    PgFunction(PgFunc),
}
