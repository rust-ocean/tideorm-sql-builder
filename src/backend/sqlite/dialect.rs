//! SQLite's dialect: JSON as text read through `json_each`, arrays as JSON
//! documents, and positional markers.

use super::{SqliteQueryBuilder, json};
use crate::backend::bound_sql::bound;
use crate::backend::dialect::{
    ArrayOperator, BoundSql, JsonContainment, JsonExistence, JsonPathStep, LONG_LIST, SqlDialect,
    all_elements, any_element, array_contained_by, builder_dialect, document, existence_steps,
    in_list, json_path_text, json_text, negated, unknown_when_null,
};
use crate::{Expr, Value, error::RenderError};

/// The alias of a JSON array's `json_each` rows.
const ELEMENT: &str = "tideorm_element";

impl SqlDialect for SqliteQueryBuilder {
    builder_dialect!(SqliteQueryBuilder);

    /// SQLite compares JSON only as text, so the document's structure is
    /// compared instead. A NULL column matches neither the test nor its
    /// negation.
    fn json_equals(&self, column: &str, value: &serde_json::Value, negate: bool) -> BoundSql {
        let equal = json::sqlite_json_equals(column, value);
        if negate {
            bound!(format!("({column} IS NOT NULL AND NOT "), equal, ")")
        } else {
            equal
        }
    }

    fn json_containment(
        &self,
        column: &str,
        value: &serde_json::Value,
        containment: JsonContainment,
    ) -> BoundSql {
        match containment {
            JsonContainment::Contains => json::sqlite_json_contains(column, value),
            JsonContainment::ContainedBy => json::sqlite_json_contained_by(column, value),
        }
    }

    /// `json_type` is strict already, and finds a member holding JSON `null`,
    /// which `json_extract` reports as SQL `NULL`.
    fn json_exists(
        &self,
        column: &str,
        existence: JsonExistence,
        target: &str,
        negate: bool,
    ) -> Result<BoundSql, RenderError> {
        let steps = existence_steps(existence, target, "SQLite")?;
        let sql = unknown_when_null(
            column,
            bound!(
                format!("json_type({column}, "),
                Value::String(Some(json_path_text(&steps))),
                ") IS NOT NULL"
            ),
        );
        Ok(negated(sql, negate))
    }

    /// Elements are compared as JSON values, as `json_equals` compares a
    /// document: `1` is not `true`, and the string `"{\"a\":1}"` is not that
    /// object. A null in a contained-by list admits null elements, and a NULL
    /// column has no elements to test, so it is contained by nothing, as
    /// PostgreSQL's `<@` leaves it.
    fn array_predicate(
        &self,
        column: &str,
        operator: ArrayOperator,
        elements: Vec<Value>,
        _null_allowed: bool,
    ) -> BoundSql {
        let checks: Vec<BoundSql> = elements
            .iter()
            .map(|element| json::sqlite_json_element_equals(ELEMENT, &document(element)))
            .collect();
        let rows = format!("json_each({column}) AS {ELEMENT}");
        let has_elements = |checks: Vec<BoundSql>| {
            checks
                .into_iter()
                .map(|equal| bound!(format!("EXISTS (SELECT 1 FROM {rows} WHERE "), equal, ")"))
                .collect()
        };
        let sql = match operator {
            ArrayOperator::Contains => all_elements(column, has_elements(checks)),
            ArrayOperator::Overlaps => any_element(has_elements(checks)),
            ArrayOperator::ContainedBy => {
                let offending = (!checks.is_empty())
                    .then(|| bound!("NOT (", BoundSql::join(checks, " OR "), ")"));
                array_contained_by(column, &rows, offending)
            }
        };
        // `json_each` also enumerates objects and scalar documents. Array
        // operators must never widen reads or writes to those values.
        bound!(format!("(json_type({column}) = 'array' AND ("), sql, "))")
    }

    /// A document that is no array becomes the first element of one, as
    /// MySQL's `JSON_ARRAY_APPEND` wraps it.
    fn array_append(&self, column: &str, element: Value) -> BoundSql {
        let text = json_text(&document(&element));
        bound!(
            format!("CASE WHEN {column} IS NULL THEN json_array(json("),
            text.clone(),
            format!(
                ")) WHEN json_type({column}) = 'array' THEN json_insert({column}, '$[#]', json("
            ),
            text.clone(),
            format!(")) ELSE json_array(json({column}), json("),
            text,
            ")) END"
        )
    }

    /// The kept elements are extracted as JSON text, not `json_each.value`:
    /// SQL REAL rounds large integers, and rebuilding from it would corrupt
    /// untouched values.
    fn array_remove(&self, column: &str, source: &str, element: Value) -> BoundSql {
        let equal = json::sqlite_json_element_equals(ELEMENT, &document(&element));
        let kept = format!("json({ELEMENT}.json -> {ELEMENT}.fullkey)");
        bound!(
            format!(
                "CASE WHEN {column} IS NULL THEN NULL WHEN json_type({column}) <> 'array' THEN {column} \
                 ELSE (SELECT json_group_array({kept} ORDER BY {ELEMENT}.key) \
                 FROM json_each({source}) AS {ELEMENT} WHERE NOT "
            ),
            equal,
            ") END"
        )
    }

    /// SQLite's own `json_set` would create missing parents and ignore an
    /// index past an array's end. `json_type` of a missing parent is NULL,
    /// which `IS NOT` counts as another type.
    fn json_set(&self, column: &str, path: &[JsonPathStep], value: &serde_json::Value) -> BoundSql {
        let text = |steps: &[JsonPathStep], suffix: &str| {
            Value::String(Some(format!("{}{suffix}", json_path_text(steps))))
        };
        let Some((last, parent)) = path.split_last() else {
            return BoundSql::sql(column);
        };
        let document = json_text(value);
        match last {
            JsonPathStep::Member(_) => bound!(
                format!("CASE WHEN json_type({column}, "),
                text(parent, ""),
                format!(") IS NOT 'object' THEN {column} ELSE json_set({column}, "),
                text(path, ""),
                ", json(",
                document,
                ")) END"
            ),
            JsonPathStep::Element(index) => bound!(
                format!("CASE WHEN json_type({column}, "),
                text(parent, ""),
                format!(") IS NOT 'array' THEN {column} WHEN json_array_length({column}, "),
                text(parent, ""),
                format!(") > {index} THEN json_set({column}, "),
                text(path, ""),
                ", json(",
                document.clone(),
                format!(")) ELSE json_insert({column}, "),
                text(parent, "[#]"),
                ", json(",
                document,
                ")) END"
            ),
        }
    }

    fn open_limit(&self) -> Option<&'static str> {
        Some("-1")
    }

    fn exact_key(&self, column: &str) -> String {
        format!("CAST({column} AS BLOB)")
    }

    /// `LIKE` stops at a stored NUL. Blob slicing and search after ASCII case
    /// folding (where `LIKE` folds case) preserve the whole text, NULs
    /// included; `lower()` stops at a NUL in `literal`, which is refused.
    fn literal_match(
        &self,
        column: &str,
        literal: &str,
        leading: bool,
        trailing: bool,
    ) -> Result<BoundSql, RenderError> {
        if literal.contains('\0') {
            return Err(RenderError::Unsupported(
                "SQLite literal text filters cannot represent a NUL character".into(),
            ));
        }
        let test = match (leading, trailing) {
            (true, true) => "instr(hay, needle) > 0",
            (false, true) => "substr(hay, 1, length(needle)) = needle",
            (true, false) => "length(needle) = 0 OR substr(hay, -length(needle)) = needle",
            (false, false) => "hay = needle",
        };
        Ok(bound!(
            format!(
                "(SELECT CASE WHEN hay IS NOT NULL THEN ({test}) END FROM \
                 (SELECT CAST(CASE WHEN 'a' LIKE 'A' THEN lower(raw_hay) ELSE raw_hay END AS BLOB) AS hay, \
                 CAST(CASE WHEN 'a' LIKE 'A' THEN lower(raw_needle) ELSE raw_needle END AS BLOB) AS needle \
                 FROM (SELECT {column} AS raw_hay, "
            ),
            Value::String(Some(literal.to_owned())),
            " AS raw_needle)))"
        ))
    }

    /// A long text list is read from one JSON array parameter. Other types
    /// keep their bound form: a UUID is stored as a blob, and a timestamp as
    /// text in a format its JSON form does not share.
    fn in_list(&self, column: Expr, values: Vec<Value>, negated: bool) -> Expr {
        if values.len() > LONG_LIST
            && let Some(texts) = values
                .iter()
                .map(|value| match value {
                    Value::String(Some(text)) => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
        {
            let operator = if negated { " NOT IN" } else { " IN" };
            return bound!(
                format!("{operator} (SELECT value FROM json_each("),
                Value::String(Some(serde_json::to_string(&texts).unwrap_or_default())),
                "))"
            )
            .following(column);
        }
        in_list(column, values, negated)
    }
}
