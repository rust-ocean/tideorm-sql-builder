//! Conditions that fold to a constant.

use super::{Condition, ConditionExpression, ConditionType};
use crate::{
    BinOper, Expr, UnOper, Value,
    lexer::{Dialect, Kind, tokens},
};

impl Condition {
    /// Whether the condition holds for every row, whatever the row holds:
    /// it folds to TRUE, as an empty [`Condition::all`] does, or `x NOT IN ()`,
    /// or an OR with a TRUE branch.
    ///
    /// A condition that only might hold everywhere (`col = col`) is not
    /// trivially true; what is not a constant is left unknown.
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// assert!(Cond::all().is_trivially_true());
    /// assert!(Cond::any().add(Expr::col("id").eq(1)).add(Expr::val(true)).is_trivially_true());
    /// assert!(!Cond::all().add(Expr::col("id").eq(1)).is_trivially_true());
    /// ```
    pub fn is_trivially_true(&self) -> bool {
        self.constant() == Some(true)
    }

    /// The value the condition folds to, if it is a constant.
    fn constant(&self) -> Option<bool> {
        let values = self.conditions.iter().map(|condition| match condition {
            ConditionExpression::Condition(condition) => condition.constant(),
            ConditionExpression::Expr(expr) => expr_constant(expr),
        });
        let folded = match self.condition_type {
            ConditionType::All => all(values),
            ConditionType::Any => any(values),
        };
        folded.map(|value| value != self.negate)
    }
}

/// AND of `values`: FALSE when one is, TRUE when all are, else unknown.
fn all(values: impl IntoIterator<Item = Option<bool>>) -> Option<bool> {
    let mut known = Some(true);
    for value in values {
        match value {
            Some(false) => return Some(false),
            Some(true) => {}
            None => known = None,
        }
    }
    known
}

/// OR of `values`: TRUE when one is, FALSE when all are, else unknown.
fn any(values: impl IntoIterator<Item = Option<bool>>) -> Option<bool> {
    all(values.into_iter().map(|value| value.map(|value| !value))).map(|value| !value)
}

fn expr_constant(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Value(Value::Bool(Some(value))) | Expr::Constant(Value::Bool(Some(value))) => {
            Some(*value)
        }
        Expr::Unary(UnOper::Not, inner) => expr_constant(inner).map(|value| !value),
        Expr::Binary(left, operator, right) => match (operator, right.as_ref()) {
            (BinOper::In, Expr::Tuple(list)) if list.is_empty() => Some(false),
            (BinOper::NotIn, Expr::Tuple(list)) if list.is_empty() => Some(true),
            (BinOper::And, _) => all([expr_constant(left), expr_constant(right)]),
            (BinOper::Or, _) => any([expr_constant(left), expr_constant(right)]),
            (BinOper::Equal, _) => equal(left, right),
            (BinOper::NotEqual, _) => equal(left, right).map(|value| !value),
            _ => None,
        },
        Expr::Custom(sql) => custom_constant(sql),
        _ => None,
    }
}

/// Whether two literal values are equal; unknown for NULL, a column or
/// values of different types, which the database may compare otherwise.
fn equal(left: &Expr, right: &Expr) -> Option<bool> {
    let literal = |expr: &Expr| match expr {
        Expr::Value(value) | Expr::Constant(value) if *value != value.as_null() => {
            Some(value.clone())
        }
        _ => None,
    };
    let (left, right) = (literal(left)?, literal(right)?);
    (std::mem::discriminant(&left) == std::mem::discriminant(&right)).then(|| left == right)
}

/// The constant a fragment of SQL text spells: `TRUE`, `1`, `1 = 1`,
/// `TRUE = TRUE` or their false forms, parenthesized or not. Any other text
/// is unknown.
fn custom_constant(sql: &str) -> Option<bool> {
    let mut spelled = String::new();
    for token in tokens(sql, Dialect::Any) {
        match token.kind {
            Kind::Space => {}
            Kind::Symbol if matches!(token.text, "(" | ")") => {}
            Kind::Word | Kind::Symbol => spelled.push_str(&token.text.to_ascii_uppercase()),
            _ => return None,
        }
    }
    match spelled.as_str() {
        "TRUE" | "1" | "1=1" | "TRUE=TRUE" => Some(true),
        "FALSE" | "0" | "0=1" | "1=0" | "1=2" | "FALSE=TRUE" | "TRUE=FALSE" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/condition_constant_tests.rs"]
mod tests;
