//! PostgreSQL's dialect: `jsonb` operators, native arrays and numbered markers.
//!
//! Array predicates test each value with `value = ANY(column)` rather than
//! against an `ARRAY[..]` of them, which lets PostgreSQL infer each
//! parameter's type from the column's element type.

use super::PostgresQueryBuilder;
use crate::backend::bound_sql::bound;
use crate::backend::dialect::{
    ArrayOperator, BoundSql, JsonContainment, JsonExistence, JsonPathStep, LONG_LIST, SqlDialect,
    all_elements, any_element, array_contained_by, builder_dialect, in_list, json_path_text,
    json_text, negated, parse_json_path, unknown_when_null,
};
use crate::extension::postgres::PgFunc;
use crate::{ArrayType, Expr, ExprTrait, Value, error::RenderError};

/// The alias of the `unnest(..)` derived table in the contained-by test,
/// together with the name of its single column.
const ARRAY_ELEMENT: &str = "tideorm_array_element";

impl SqlDialect for PostgresQueryBuilder {
    builder_dialect!(PostgresQueryBuilder);

    fn native_arrays(&self) -> bool {
        true
    }

    fn exact_key(&self, column: &str) -> String {
        format!("CAST({column} AS TEXT) COLLATE \"C\"")
    }

    /// A long list of one type is one array parameter, so any length fits in
    /// a statement and every length shares one prepared statement.
    fn in_list(&self, column: Expr, values: Vec<Value>, negated: bool) -> Expr {
        if values.len() > LONG_LIST
            && let Some(array_type) = shared_array_type(&values)
        {
            let array = Expr::val(Value::Array(array_type, Some(Box::new(values))));
            return if negated {
                column.ne(PgFunc::all(array))
            } else {
                column.eq(PgFunc::any(array))
            };
        }
        in_list(column, values, negated)
    }

    /// The `json` type has no `=` at all, so the column is read as `jsonb`.
    fn json_equals(&self, column: &str, value: &serde_json::Value, negated: bool) -> BoundSql {
        let operator = if negated { " <> " } else { " = " };
        bound!(jsonb(column), operator, Value::from(value.clone()))
    }

    fn json_containment(
        &self,
        column: &str,
        value: &serde_json::Value,
        containment: JsonContainment,
    ) -> BoundSql {
        let operator = match containment {
            JsonContainment::Contains => " @> ",
            JsonContainment::ContainedBy => " <@ ",
        };
        bound!(jsonb(column), operator, Value::from(value.clone()))
    }

    /// A key is tested with `?`, which also finds a string element of an
    /// array, so the document must be an object first; a path with `@?` in
    /// `strict` mode, where the default `lax` mode would look into arrays and
    /// read a scalar as an array of one. A path the other backends cannot
    /// express is read as the JSONPath written.
    fn json_exists(
        &self,
        column: &str,
        existence: JsonExistence,
        target: &str,
        negate: bool,
    ) -> Result<BoundSql, RenderError> {
        let jsonb = jsonb(column);
        let text = |text: String| Value::String(Some(text));
        let sql = match existence {
            JsonExistence::Key => unknown_when_null(
                column,
                bound!(
                    format!("(jsonb_typeof({jsonb}) = 'object' AND {jsonb} ? "),
                    text(target.to_string()),
                    ")"
                ),
            ),
            // A step strict mode refuses is suppressed by `@?`, which then
            // reports no answer; it is a missing member.
            JsonExistence::Path => match parse_json_path(target) {
                Some(steps) => unknown_when_null(
                    column,
                    bound!(
                        format!("COALESCE({jsonb} @? ("),
                        text(format!("strict {}", json_path_text(&steps))),
                        "::jsonpath), FALSE)"
                    ),
                ),
                None => bound!(
                    format!("{jsonb} @? ("),
                    text(target.to_string()),
                    "::jsonpath)"
                ),
            },
        };
        Ok(negated(sql, negate))
    }

    fn array_predicate(
        &self,
        column: &str,
        operator: ArrayOperator,
        elements: Vec<Value>,
        null_allowed: bool,
    ) -> BoundSql {
        let checks = || {
            elements
                .iter()
                .map(|element| bound!(element.clone(), format!(" = ANY({column})")))
                .collect()
        };
        match operator {
            ArrayOperator::Contains => all_elements(column, checks()),
            ArrayOperator::Overlaps => any_element(checks()),
            ArrayOperator::ContainedBy => contained_by(column, elements, null_allowed),
        }
    }

    fn array_append(&self, column: &str, element: Value) -> BoundSql {
        bound!(format!("array_append({column}, "), element, ")")
    }

    fn array_remove(&self, column: &str, _source: &str, element: Value) -> BoundSql {
        bound!(format!("array_remove({column}, "), element, ")")
    }

    /// `jsonb_set` reads an index step of an object as a key and fails the
    /// statement on a member step of an array, so each step's parent is
    /// checked to be the object or array the step needs first.
    fn json_set(&self, column: &str, path: &[JsonPathStep], value: &serde_json::Value) -> BoundSql {
        let jsonb = jsonb(column);
        if path.is_empty() {
            return BoundSql::sql(jsonb);
        }
        let guards = path.iter().enumerate().map(|(index, step)| {
            let kind = match step {
                JsonPathStep::Member(_) => "object",
                JsonPathStep::Element(_) => "array",
            };
            bound!(
                format!("jsonb_typeof({jsonb} #> "),
                Value::String(Some(path_array(&path[..index]))),
                format!("::text[]) = '{kind}'")
            )
        });
        bound!(
            "CASE WHEN ",
            BoundSql::join(guards, " AND "),
            format!(" THEN jsonb_set({jsonb}, "),
            Value::String(Some(path_array(path))),
            "::text[], CAST(",
            json_text(value),
            format!(" AS jsonb)) ELSE {jsonb} END")
        )
    }
}

/// `column` as `jsonb`, which PostgreSQL's JSON operators require: a `json`
/// column has none of them. On a `jsonb` column the cast is dropped at
/// planning, so its indexes still apply.
fn jsonb(column: &str) -> String {
    format!("({column})::jsonb")
}

/// A JSON path as the `text[]` literal `jsonb_set` takes.
fn path_array(steps: &[JsonPathStep]) -> String {
    let elements: Vec<String> = steps
        .iter()
        .map(|step| {
            let text = match step {
                JsonPathStep::Member(key) => key.clone(),
                JsonPathStep::Element(index) => index.to_string(),
            };
            format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
        })
        .collect();
    format!("{{{}}}", elements.join(","))
}

/// `column <@ ARRAY[..]`, `elements` being the list's non-NULL values and
/// `null_allowed` whether the list also holds a NULL.
///
/// An empty list leaves only the empty array contained, and a NULL element is
/// contained only when the list holds a NULL. A NULL *element* makes
/// `element NOT IN (..)` evaluate to NULL rather than TRUE, so the offending
/// row would not be counted; `element IS NULL OR ..` counts it instead. A NULL
/// in the list is left out of `NOT IN` for the same reason: it would make
/// every element look contained.
fn contained_by(column: &str, elements: Vec<Value>, null_allowed: bool) -> BoundSql {
    let element = format!("{ARRAY_ELEMENT}.element");
    let empty = elements.is_empty();
    let list = bound!(
        format!("{element} NOT IN ("),
        BoundSql::join(elements.into_iter().map(|value| bound!(value)), ", "),
        ")"
    );
    let offending = match (empty, null_allowed) {
        (true, false) => None,
        (true, true) => Some(BoundSql::sql(format!("{element} IS NOT NULL"))),
        (false, false) => Some(bound!(format!("{element} IS NULL OR "), list)),
        (false, true) => Some(list),
    };
    array_contained_by(
        column,
        &format!("unnest({column}) AS {ARRAY_ELEMENT}(element)"),
        offending,
    )
}

/// The array type of `values`, when they share one.
fn shared_array_type(values: &[Value]) -> Option<ArrayType> {
    let array_type = values.first()?.array_type();
    values
        .iter()
        .all(|value| value.array_type() == array_type)
        .then_some(array_type)
}

#[cfg(test)]
#[path = "../../../tests/unit/postgres_dialect_tests.rs"]
mod tests;
