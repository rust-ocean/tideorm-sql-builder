//! The SQL a backend writes beyond its statements.
//!
//! TideORM's query builder renders JSON and array predicates, JSON column
//! updates, window defaults, parameter markers and literals as text, and asks
//! the backend's [`SqlDialect`] for each piece, so every difference between the
//! dialects lives in the backend that speaks it. A piece with values comes
//! back as a [`BoundSql`], its values standing where they are bound.

use super::bound_sql::bound;
pub use super::bound_sql::{BoundPart, BoundPiece, BoundSql};
use crate::{Expr, ExprTrait, Quote, Value, error::RenderError};

pub(crate) use super::json_path::{JsonPathStep, json_path_text, parse_json_path};

/// A predicate that holds for no row.
///
/// A condition that cannot hold (an overlap with no values, a search with no
/// word) renders as it rather than leaving the WHERE clause.
pub const MATCH_NOTHING: &str = "0 = 1";

/// Lists longer than this are long. Every backend caps a statement at 32,766
/// or 65,535 bind parameters, so [`SqlDialect::in_list`] writes a long list
/// without binding each value.
pub const LONG_LIST: usize = 1_000;

/// The escape character of every `LIKE` pattern [`SqlDialect::literal_match`]
/// writes.
///
/// Deliberately **not** a backslash: SQLite string literals do not process it
/// (so `'\\'` is a two-character escape argument SQLite rejects), and MySQL's
/// handling flips with `NO_BACKSLASH_ESCAPES`. `!` is a single character in
/// all three dialects and is not special inside a `LIKE` pattern.
const LIKE_ESCAPE: char = '!';

/// Which way a JSON containment test reads, as PostgreSQL's `@>` and `<@` do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonContainment {
    /// The column holds every key and element of the value.
    Contains,
    /// The value holds every key and element of the column.
    ContainedBy,
}

/// What a JSON existence predicate looks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonExistence {
    /// A top-level member, named by a plain key.
    Key,
    /// A JSON path such as `$.user.name`.
    Path,
}

/// An array predicate, read as PostgreSQL's array operators read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayOperator {
    /// The column holds every listed value (`@>`).
    Contains,
    /// The list holds every element of the column (`<@`).
    ContainedBy,
    /// The column holds some listed value (`&&`).
    Overlaps,
}

/// The SQL dialect of a backend, for SQL written as text.
///
/// `column` arguments are SQL already quoted. JSON is compared as PostgreSQL
/// compares `jsonb` on every backend: as documents, not text, so key order
/// and spacing do not count and `1` equals `1.0`.
pub trait SqlDialect: Send + Sync {
    /// The quotes around an identifier.
    fn identifier_quote(&self) -> Quote;

    /// Append `name` to `out` as a quoted identifier, doubling any quote inside it.
    fn push_identifier(&self, out: &mut String, name: &str) {
        super::write_identifier(out, self.identifier_quote(), name);
    }

    /// The marker of the `index`-th (1-based) bound value.
    fn marker(&self, index: usize) -> String;

    /// Whether markers are numbered (`$1`).
    ///
    /// A fragment with numbered markers is renumbered when it is spliced
    /// behind other values; one with positional `?` markers is not.
    fn numbered_markers(&self) -> bool;

    /// SQL written with this dialect's markers, such as a raw fragment, as an
    /// expression binding `values` to its markers in order. The dialect's own
    /// pieces need no reading: a [`BoundSql`] is an expression as it is.
    ///
    /// A marker without a value, or a value without a marker, is an error.
    fn expression(&self, sql: String, values: Vec<Value>) -> Result<Expr, RenderError>;

    /// `sql` with `values` written in place of its markers, for display.
    fn inline_values(&self, sql: &str, values: &[Value]) -> Result<String, RenderError>;

    /// `text` as a quoted string literal, escaped as [`inline_values`]
    /// writes one, for SQL that cannot bind it: DDL, or a constant an index
    /// expression must match.
    ///
    /// [`inline_values`]: Self::inline_values
    fn string_literal(&self, text: &str) -> String;

    /// Whether an array column is the database's own array type.
    ///
    /// Its elements are then bound as values of the element type; elsewhere an
    /// array is a JSON document and its elements are JSON values
    /// (`Value::Json`).
    fn native_arrays(&self) -> bool {
        false
    }

    /// `column` holds the JSON document `value`, or does not when `negated`.
    ///
    /// A NULL column matches neither the test nor its negation.
    fn json_equals(&self, column: &str, value: &serde_json::Value, negated: bool) -> BoundSql;

    /// A JSON containment test of `column` against `value`.
    fn json_containment(
        &self,
        column: &str,
        value: &serde_json::Value,
        containment: JsonContainment,
    ) -> BoundSql;

    /// A JSON key- or path-existence test, or its negation.
    ///
    /// Every backend reads a path strictly: a member step finds a member of an
    /// object only, never of the objects inside an array, and an index step an
    /// element of an array only. A key is the one-member path that names it,
    /// so it exists only in an object. A member holding JSON `null` exists,
    /// and a NULL column is unknown, so that the negation does not match it
    /// either. A path with steps the backend cannot express is refused:
    /// matching nothing, both the test and its negation, would hide the
    /// mistake.
    fn json_exists(
        &self,
        column: &str,
        existence: JsonExistence,
        target: &str,
        negated: bool,
    ) -> Result<BoundSql, RenderError>;

    /// An array predicate on `column` against the list `elements`.
    ///
    /// `elements` are bound as [`native_arrays`](Self::native_arrays) says,
    /// and `null_allowed` says whether a contained-by list of native elements
    /// also held a NULL, which `elements` leaves out.
    fn array_predicate(
        &self,
        column: &str,
        operator: ArrayOperator,
        elements: Vec<Value>,
        null_allowed: bool,
    ) -> BoundSql;

    /// `column` with `element` appended; a NULL column starts a new array.
    fn array_append(&self, column: &str, element: Value) -> BoundSql;

    /// `column` without the elements equal to `element`.
    ///
    /// `source` is the column qualified with its table, for a subquery that
    /// reads its elements.
    fn array_remove(&self, column: &str, source: &str, element: Value) -> BoundSql;

    /// `column` with the member or element at `path` set to `value`.
    ///
    /// A path whose parent is missing (read strictly, as
    /// [`json_exists`](Self::json_exists) reads a path), or is not the object
    /// its member step needs or the array its index step needs, leaves the
    /// document as it is, and an index past an array's end appends the value.
    fn json_set(&self, column: &str, path: &[JsonPathStep], value: &serde_json::Value) -> BoundSql;

    /// `LAG` or `LEAD` (`function`) with a default, over the window `over`.
    fn window_default(
        &self,
        function: &str,
        column: &str,
        offset: &str,
        default: &str,
        over: &str,
    ) -> String {
        format!("{function}({column}, {offset}, {default}) OVER ({over})")
    }

    /// The limit an offset alone takes, where the dialect has no bare `OFFSET`.
    fn open_limit(&self) -> Option<&'static str> {
        None
    }

    /// Whether an `IN (..)` subquery may carry a `LIMIT`.
    fn limit_in_subquery(&self) -> bool {
        true
    }

    /// Whether a window frame may count `GROUPS`.
    fn groups_frames(&self) -> bool {
        true
    }

    /// Whether an `UPDATE` takes `ORDER BY` and `LIMIT` itself. Where it does
    /// not, a capped update is scoped through a primary-key subquery; the
    /// builder renders an [`UpdateStatement`](crate::UpdateStatement)'s limit
    /// as it is told.
    fn update_limit(&self) -> bool {
        false
    }

    /// `column` as a value that compares byte for byte, whatever its
    /// collation, so two spellings a case- or accent-insensitive collation
    /// matches stay apart.
    fn exact_key(&self, column: &str) -> String;

    /// A query's ` LIMIT n OFFSET ?`.
    ///
    /// The limit is written into the SQL, where SQLite 3.50+ would recompile a
    /// statement with a bound LIMIT on every run, and the offset is bound, as a
    /// `u64` like a select's, so paging reuses one statement.
    fn limit_offset(&self, limit: Option<u64>, offset: Option<u64>) -> BoundSql {
        let mut bound = match (limit, offset) {
            (Some(limit), _) => BoundSql::sql(format!(" LIMIT {limit}")),
            (None, Some(_)) => match self.open_limit() {
                Some(open) => BoundSql::sql(format!(" LIMIT {open}")),
                None => BoundSql::default(),
            },
            (None, None) => BoundSql::default(),
        };
        if let Some(offset) = offset {
            bound.push(" OFFSET ").push(Value::from(offset));
        }
        bound
    }

    /// A test that `column` holds the text `literal`, preceded by anything
    /// when `leading` and followed by anything when `trailing`: a `LIKE`
    /// pattern with the literal's `%`, `_` and escape character escaped.
    /// Refused when the backend cannot test `literal` as it is.
    fn literal_match(
        &self,
        column: &str,
        literal: &str,
        leading: bool,
        trailing: bool,
    ) -> Result<BoundSql, RenderError> {
        let mut pattern = String::with_capacity(literal.len() + 2);
        if leading {
            pattern.push('%');
        }
        for character in literal.chars() {
            if matches!(character, LIKE_ESCAPE | '%' | '_') {
                pattern.push(LIKE_ESCAPE);
            }
            pattern.push(character);
        }
        if trailing {
            pattern.push('%');
        }
        Ok(bound!(
            format!("{column} LIKE "),
            Value::String(Some(pattern)),
            format!(" ESCAPE '{LIKE_ESCAPE}'")
        ))
    }

    /// `column IN values`, or `NOT IN` when `negated`. A long all-integer
    /// list is written as literals.
    fn in_list(&self, column: Expr, values: Vec<Value>, negated: bool) -> Expr {
        in_list(column, values, negated)
    }
}

/// `column IN values` (`NOT IN` when `negated`), a [`LONG_LIST`] of integers
/// written as literals: integers carry nothing but digits and a sign.
pub(crate) fn in_list(column: Expr, values: Vec<Value>, negated: bool) -> Expr {
    let literals = values.len() > LONG_LIST
        && values.iter().all(|value| {
            matches!(
                value,
                Value::TinyInt(Some(_))
                    | Value::SmallInt(Some(_))
                    | Value::Int(Some(_))
                    | Value::BigInt(Some(_))
                    | Value::TinyUnsigned(Some(_))
                    | Value::SmallUnsigned(Some(_))
                    | Value::Unsigned(Some(_))
                    | Value::BigUnsigned(Some(_))
            )
        });
    let list = values.into_iter().map(|value| {
        if literals {
            Expr::Constant(value)
        } else {
            Expr::Value(value)
        }
    });
    if negated {
        column.is_not_in(list)
    } else {
        column.is_in(list)
    }
}

/// The six [`SqlDialect`] methods a builder answers itself.
macro_rules! builder_dialect {
    ($builder:expr) => {
        fn identifier_quote(&self) -> $crate::Quote {
            $crate::QuotedBuilder::quote(&$builder)
        }

        fn marker(&self, index: usize) -> String {
            match $crate::QueryBuilder::placeholder(&$builder) {
                (marker, true) => format!("{marker}{index}"),
                (marker, false) => marker.to_string(),
            }
        }

        fn numbered_markers(&self) -> bool {
            $crate::QueryBuilder::placeholder(&$builder).1
        }

        fn expression(
            &self,
            sql: String,
            values: Vec<$crate::Value>,
        ) -> Result<$crate::Expr, $crate::error::RenderError> {
            $crate::Expr::cust_with_values(sql, values, &$builder)
        }

        fn inline_values(
            &self,
            sql: &str,
            values: &[$crate::Value],
        ) -> Result<String, $crate::error::RenderError> {
            $crate::inject_parameters(sql, values, &$builder)
        }

        fn string_literal(&self, text: &str) -> String {
            let mut literal = String::with_capacity(text.len() + 2);
            $crate::QueryBuilder::write_string_quoted(&$builder, text, &mut literal);
            literal
        }
    };
}
pub(crate) use builder_dialect;

/// `test`, unknown for a NULL column, which matches neither a JSON test nor
/// its negation.
pub(crate) fn unknown_when_null(column: &str, test: BoundSql) -> BoundSql {
    bound!(
        format!("CASE WHEN {column} IS NOT NULL THEN "),
        test,
        " END"
    )
}

/// `sql`, or its negation when `negated`.
pub(crate) fn negated(sql: BoundSql, negated: bool) -> BoundSql {
    if negated {
        bound!("NOT (", sql, ")")
    } else {
        sql
    }
}

/// A JSON value bound as its JSON text, for backends that parse it server-side.
pub(crate) fn json_text(value: &serde_json::Value) -> Value {
    Value::String(Some(value.to_string()))
}

/// The JSON document a JSON-array element binds: a JSON value as itself, SQL
/// NULL as `null`, and anything else as its JSON reading.
pub(crate) fn document(value: &Value) -> serde_json::Value {
    match value {
        Value::Json(Some(json)) => json.as_ref().clone(),
        other => crate::value_to_json_value(other).unwrap_or(serde_json::Value::Null),
    }
}

/// The steps a JSON existence test follows: a key's one member, or a path's,
/// refused when it is not a path `backend` can read.
pub(crate) fn existence_steps(
    existence: JsonExistence,
    target: &str,
    backend: &str,
) -> Result<Vec<JsonPathStep>, RenderError> {
    match existence {
        JsonExistence::Key => Ok(vec![JsonPathStep::Member(target.to_string())]),
        JsonExistence::Path => parse_json_path(target).ok_or_else(|| {
            RenderError::Unsupported(format!(
                "'{target}' is not a JSON path {backend} can test; write `$` followed by \
                 `.key`, `.\"quoted key\"` or `[index]` steps"
            ))
        }),
    }
}

/// Every one of the element `checks` holds: what containment of a list
/// tests. A list of no elements is contained by every non-NULL array.
pub(crate) fn all_elements(column: &str, checks: Vec<BoundSql>) -> BoundSql {
    joined(checks, " AND ", &format!("{column} IS NOT NULL"))
}

/// One of the element `checks` holds: what an overlap with a list tests.
/// Nothing overlaps a list of no elements.
pub(crate) fn any_element(checks: Vec<BoundSql>) -> BoundSql {
    joined(checks, " OR ", MATCH_NOTHING)
}

/// `checks` joined by `combine`, parenthesized; `empty` when there are none.
pub(crate) fn joined(checks: Vec<BoundSql>, combine: &str, empty: &str) -> BoundSql {
    if checks.is_empty() {
        return BoundSql::sql(empty);
    }
    bound!("(", BoundSql::join(checks, combine), ")")
}

/// "Every element of `column` is in the list" over `source`, a table of its
/// elements, of which `offending` selects those the list does not hold
/// (`None`: every element offends).
///
/// A NULL column has no elements, so a bare `NOT EXISTS` is TRUE for it —
/// where `NULL <@ ARRAY[..]` is NULL and matches nothing. The explicit
/// `IS NOT NULL` restores that. Without it a NULL array column silently
/// matches, which widens a SELECT and, on a mutation, the set of rows
/// written or deleted.
pub(crate) fn array_contained_by(
    column: &str,
    source: &str,
    offending: Option<BoundSql>,
) -> BoundSql {
    let rows = format!("({column} IS NOT NULL AND NOT EXISTS (SELECT 1 FROM {source}");
    match offending {
        None => BoundSql::sql(format!("{rows}))")),
        Some(offending) => bound!(format!("{rows} WHERE "), offending, "))"),
    }
}

#[cfg(test)]
#[path = "../../tests/unit/in_list_tests.rs"]
mod tests;
