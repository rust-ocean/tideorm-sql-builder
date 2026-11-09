use std::borrow::Cow;

mod constructors;
mod custom;
pub use custom::CustomExpr;
pub(crate) use custom::CustomExprPart;
mod conversions;
mod subqueries;

use crate::*;

/// An arbitrary, dynamically-typed SQL expression.
///
/// It can be used in select fields, where clauses and many other places.
///
/// More concreterly, under the hood [`Expr`]s can be:
///
/// - Rust values
/// - SQL identifiers
/// - SQL function calls
/// - various operators and sub-queries
///
/// If something is not supported here, look into [`BinOper::Custom`],
/// [`Func::cust`], or [`Expr::cust*`][`Expr::cust_with_values`] as a
/// workaround, and consider reporting your issue.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Column(ColumnRef),
    Tuple(Vec<Expr>),
    Unary(UnOper, Box<Expr>),
    FunctionCall(FunctionCall),
    Binary(Box<Expr>, BinOper, Box<Expr>),
    SubQuery(Option<SubQueryOper>, Box<SubQueryStatement>),
    Value(Value),
    Values(Vec<Value>),
    Custom(Cow<'static, str>),
    CustomWithExpr(CustomExpr),
    /// A construction error deferred until execution.
    Invalid(crate::error::RenderError),
    Keyword(Keyword),
    AsEnum(DynIden, Box<Expr>),
    Case(Box<CaseStatement>),
    Constant(Value),
    TypeName(TypeRef),
}
