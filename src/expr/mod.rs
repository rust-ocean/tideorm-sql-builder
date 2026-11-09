//! Building blocks of SQL statements.
//!
//! [`Expr`] is an arbitrary, dynamically-typed SQL expression.
//! It can be used in select fields, where clauses and many other places.
//!
//! [`ExprTrait`] provides "operator" methods for building expressions.

// Intentionally not `pub`. They only export a single item each.
// It's a mechanical split to manage LoC.
mod r#enum;
mod r#trait;

pub(crate) use r#enum::CustomExprPart;
pub use r#enum::{CustomExpr, Expr};
pub use r#trait::ExprTrait;

/// Another name for [`Expr`].
pub type SimpleExpr = Expr;
