//! SQL text with the values it binds in place.

use super::SqlDialect;
use crate::{
    Expr, Value,
    expr::{CustomExpr, CustomExprPart},
};

/// A piece of SQL with the values it binds standing where they are bound.
///
/// No marker is written until the piece renders: an [`Expr`] made from it
/// binds each value in place, and [`build_after`](Self::build_after) numbers
/// the markers after the values a statement already binds, so its text is
/// never read back for markers.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoundSql {
    parts: Vec<BoundPart>,
}

/// One part of a [`BoundSql`].
#[derive(Debug, Clone, PartialEq)]
pub enum BoundPart {
    /// SQL text.
    Sql(String),
    /// A value, bound where it stands.
    Value(Value),
}

/// What a [`BoundSql`] is built from: text, a [`Value`] bound in place, or
/// another [`BoundSql`].
pub trait BoundPiece {
    /// Append `self` to `bound`.
    fn append_to(self, bound: &mut BoundSql);
}

impl BoundPiece for &str {
    fn append_to(self, bound: &mut BoundSql) {
        if self.is_empty() {
            return;
        }
        match bound.parts.last_mut() {
            Some(BoundPart::Sql(text)) => text.push_str(self),
            _ => bound.parts.push(BoundPart::Sql(self.to_owned())),
        }
    }
}

impl BoundPiece for String {
    fn append_to(self, bound: &mut BoundSql) {
        self.as_str().append_to(bound);
    }
}

impl BoundPiece for &String {
    fn append_to(self, bound: &mut BoundSql) {
        self.as_str().append_to(bound);
    }
}

impl BoundPiece for Value {
    fn append_to(self, bound: &mut BoundSql) {
        bound.parts.push(BoundPart::Value(self));
    }
}

impl BoundPiece for BoundSql {
    fn append_to(self, bound: &mut BoundSql) {
        for part in self.parts {
            match part {
                BoundPart::Sql(text) => text.append_to(bound),
                BoundPart::Value(value) => value.append_to(bound),
            }
        }
    }
}

impl BoundSql {
    /// The SQL text `sql`, binding nothing.
    pub fn sql(sql: impl Into<String>) -> Self {
        let mut bound = Self::default();
        sql.into().append_to(&mut bound);
        bound
    }

    /// Append `piece`.
    pub fn push(&mut self, piece: impl BoundPiece) -> &mut Self {
        piece.append_to(self);
        self
    }

    /// `items` with `separator` between them.
    pub fn join(items: impl IntoIterator<Item = BoundSql>, separator: &str) -> Self {
        let mut joined = Self::default();
        for (index, item) in items.into_iter().enumerate() {
            if index > 0 {
                joined.push(separator);
            }
            joined.push(item);
        }
        joined
    }

    /// Its parts, in order.
    pub fn parts(&self) -> &[BoundPart] {
        &self.parts
    }

    /// Whether it holds no SQL and no value.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// The SQL with `dialect`'s markers, numbered after the values already
    /// in `values`, to which its own are appended.
    pub fn build_after(
        self,
        dialect: &(impl SqlDialect + ?Sized),
        values: &mut Vec<Value>,
    ) -> String {
        let mut sql = String::new();
        for part in self.parts {
            match part {
                BoundPart::Sql(text) => sql.push_str(&text),
                BoundPart::Value(value) => {
                    values.push(value);
                    sql.push_str(&dialect.marker(values.len()));
                }
            }
        }
        sql
    }

    /// The SQL with `dialect`'s markers, numbered from 1, and its values.
    pub fn build(self, dialect: &(impl SqlDialect + ?Sized)) -> (String, Vec<Value>) {
        let mut values = Vec::new();
        let sql = self.build_after(dialect, &mut values);
        (sql, values)
    }
}

/// A query builder writes into a [`BoundSql`] with each value in place, so a
/// statement it renders can be spliced into another without reading markers.
impl std::fmt::Write for BoundSql {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        self.push(text);
        Ok(())
    }
}

impl crate::SqlWriter for BoundSql {
    fn push_param<T: crate::QueryBuilder>(
        &mut self,
        value: Value,
        _: &T,
    ) -> Result<(), crate::error::RenderError> {
        self.push(value);
        Ok(())
    }
}

/// The text with a `?` standing for each value.
impl std::fmt::Display for BoundSql {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.parts {
            match part {
                BoundPart::Sql(text) => f.write_str(text)?,
                BoundPart::Value(_) => f.write_str("?")?,
            }
        }
        Ok(())
    }
}

impl BoundSql {
    /// `expr` followed by this SQL, as one expression.
    pub(crate) fn following(self, expr: Expr) -> Expr {
        let parts = std::iter::once(CustomExprPart::Expression(expr))
            .chain(self.parts.into_iter().map(CustomExprPart::from))
            .collect();
        Expr::CustomWithExpr(CustomExpr { parts })
    }
}

impl From<BoundPart> for CustomExprPart {
    fn from(part: BoundPart) -> Self {
        match part {
            BoundPart::Sql(text) => CustomExprPart::Text(text),
            BoundPart::Value(value) => CustomExprPart::Expression(Expr::Value(value)),
        }
    }
}

impl From<BoundSql> for Expr {
    fn from(bound: BoundSql) -> Self {
        let parts = bound.parts.into_iter().map(CustomExprPart::from).collect();
        Expr::CustomWithExpr(CustomExpr { parts })
    }
}

/// A [`BoundSql`] of the given pieces, in order.
macro_rules! bound {
    ($($piece:expr),* $(,)?) => {{
        let mut bound = $crate::backend::BoundSql::default();
        $( bound.push($piece); )*
        bound
    }};
}
pub(crate) use bound;
