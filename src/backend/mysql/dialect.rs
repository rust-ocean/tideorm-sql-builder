//! The dialect of MySQL and MariaDB.
//!
//! The two differ in JSON equality: MariaDB has `JSON_EQUALS` and no
//! `CAST(.. AS JSON)`, MySQL compares its JSON type with `=`. JSON values are
//! bound as JSON text and read back with `JSON_EXTRACT(?, '$')` where a
//! document is needed, which both servers parse.

use super::{
    MysqlQueryBuilder,
    json::{self, Kind},
};
use crate::backend::bound_sql::bound;
use crate::backend::dialect::{
    ArrayOperator, BoundSql, JsonContainment, JsonExistence, JsonPathStep, SqlDialect, any_element,
    builder_dialect, document, existence_steps, json_path_text, json_text, negated,
    unknown_when_null,
};
use crate::{Value, error::RenderError};

/// MariaDB's dialect: MySQL's, but for JSON equality.
#[derive(Default, Debug, Clone, Copy)]
pub struct MariaDbDialect;

/// The limit an offset alone takes: the largest MySQL accepts.
const OPEN_LIMIT: &str = "18446744073709551615";

/// One [`SqlDialect`] impl for MySQL and for MariaDB, `$mariadb` telling them apart.
macro_rules! mysql_family_dialect {
    ($dialect:ty, $mariadb:expr) => {
        impl SqlDialect for $dialect {
            builder_dialect!(MysqlQueryBuilder);

            fn json_equals(
                &self,
                column: &str,
                value: &serde_json::Value,
                negated: bool,
            ) -> BoundSql {
                json_equals(column, value, negated, $mariadb)
            }

            fn json_containment(
                &self,
                column: &str,
                value: &serde_json::Value,
                containment: JsonContainment,
            ) -> BoundSql {
                json_containment(column, value, containment)
            }

            fn json_exists(
                &self,
                column: &str,
                existence: JsonExistence,
                target: &str,
                negated: bool,
            ) -> Result<BoundSql, RenderError> {
                let backend = if $mariadb { "MariaDB" } else { "MySQL" };
                json_exists(column, existence, target, negated, backend)
            }

            fn array_predicate(
                &self,
                column: &str,
                operator: ArrayOperator,
                elements: Vec<Value>,
                _null_allowed: bool,
            ) -> BoundSql {
                array_predicate(column, operator, &elements)
            }

            fn array_append(&self, column: &str, element: Value) -> BoundSql {
                array_append(column, &element)
            }

            fn array_remove(&self, column: &str, _source: &str, element: Value) -> BoundSql {
                array_remove(column, &element, $mariadb)
            }

            fn json_set(
                &self,
                column: &str,
                path: &[JsonPathStep],
                value: &serde_json::Value,
            ) -> BoundSql {
                json_set(column, path, value)
            }

            fn window_default(
                &self,
                function: &str,
                column: &str,
                offset: &str,
                default: &str,
                over: &str,
            ) -> String {
                window_default(function, column, offset, default, over)
            }

            fn open_limit(&self) -> Option<&'static str> {
                Some(OPEN_LIMIT)
            }

            /// MySQL and MariaDB take a `LIMIT` in an `IN` subquery only a
            /// derived table further down.
            fn limit_in_subquery(&self) -> bool {
                false
            }

            fn groups_frames(&self) -> bool {
                false
            }

            fn update_limit(&self) -> bool {
                true
            }

            fn exact_key(&self, column: &str) -> String {
                format!("CAST({column} AS BINARY)")
            }
        }
    };
}

mysql_family_dialect!(MysqlQueryBuilder, false);
mysql_family_dialect!(MariaDbDialect, true);

/// MySQL compares its JSON type with `=`; a text parameter is cast to it.
/// MariaDB's `JSON_EQUALS` reads a NULL column as the `null` document, so it
/// is made unknown there, matching neither the test nor its negation.
fn json_equals(column: &str, value: &serde_json::Value, negate: bool, mariadb: bool) -> BoundSql {
    let document = json_text(value);
    if mariadb {
        let test = unknown_when_null(
            column,
            bound!(format!("JSON_EQUALS({column}, "), document, ")"),
        );
        let not = if negate { "NOT " } else { "" };
        return bound!(not, "(", test, ")");
    }
    let operator = if negate { "<>" } else { "=" };
    bound!(format!("{column} {operator} CAST("), document, " AS JSON)")
}

fn json_containment(
    column: &str,
    value: &serde_json::Value,
    containment: JsonContainment,
) -> BoundSql {
    match containment {
        JsonContainment::Contains => json::contains(column, value),
        JsonContainment::ContainedBy => json::contained_by(column, value),
    }
}

/// An index of a non-array reads as the value itself here, so each indexed
/// step also checks its parent is an array.
fn json_exists(
    column: &str,
    existence: JsonExistence,
    target: &str,
    negate: bool,
    backend: &str,
) -> Result<BoundSql, RenderError> {
    let steps = existence_steps(existence, target, backend)?;
    let mut sql = bound!(
        format!("JSON_CONTAINS_PATH({column}, 'one', "),
        Value::String(Some(json_path_text(&steps))),
        ")"
    );
    for (index, step) in steps.iter().enumerate() {
        if matches!(step, JsonPathStep::Element(_)) {
            sql.push(" AND ").push(json::guard_sql(
                column,
                Kind::Array,
                json_path_text(&steps[..index]),
            ));
        }
    }
    Ok(negated(sql, negate))
}

/// A JSON array column is contained in or contains the list as a JSON
/// document, with the guards that make `JSON_CONTAINS` read as PostgreSQL
/// does: a column holding an object is contained by no list. An overlap is a
/// containment of any one value.
fn array_predicate(column: &str, operator: ArrayOperator, elements: &[Value]) -> BoundSql {
    let documents: Vec<serde_json::Value> = elements.iter().map(document).collect();
    let bound = match operator {
        ArrayOperator::Contains => json::contains(column, &serde_json::Value::Array(documents)),
        ArrayOperator::ContainedBy => {
            json::contained_by(column, &serde_json::Value::Array(documents))
        }
        ArrayOperator::Overlaps => any_element(
            documents
                .iter()
                .map(|document| {
                    bound!(
                        format!("JSON_CONTAINS({column}, "),
                        json_text(document),
                        ")"
                    )
                })
                .collect(),
        ),
    };
    bound!(format!("(JSON_TYPE({column}) = 'ARRAY' AND ("), bound, "))")
}

/// The element is bound as its JSON text and read back as JSON, so an object
/// stays an object rather than becoming a string.
fn array_append(column: &str, element: &Value) -> BoundSql {
    bound!(
        format!("JSON_ARRAY_APPEND(COALESCE({column}, JSON_ARRAY()), '$', JSON_EXTRACT("),
        json_text(&document(element)),
        ", '$'))"
    )
}

/// Every element equal to the value goes, compared as JSON as `json_equals`
/// compares a document (`1`, `"1"` and `true` differ, an object's key order
/// does not, an array's order does), and the rest keep their order and types;
/// a NULL column stays NULL. `JSON_TABLE` reads a JSON null as SQL NULL on
/// MySQL, hence the extra test when null is what is removed.
fn array_remove(column: &str, element: &Value, mariadb: bool) -> BoundSql {
    let value = document(element);
    let equal = json_equals("tideorm_element.v", &value, false, mariadb);
    let null_guard = if value.is_null() {
        "tideorm_element.v IS NOT NULL AND "
    } else {
        ""
    };
    bound!(
        format!(
            "CASE WHEN {column} IS NULL THEN NULL WHEN JSON_TYPE({column}) <> 'ARRAY' THEN {column} \
             ELSE (SELECT COALESCE(JSON_ARRAYAGG(JSON_EXTRACT(tideorm_element.v, '$')), JSON_ARRAY()) \
             FROM JSON_TABLE({column}, '$[*]' COLUMNS (v JSON PATH '$')) AS tideorm_element \
             WHERE {null_guard}NOT COALESCE("
        ),
        equal,
        ", FALSE)) END"
    )
}

/// `JSON_SET` reads an index step of a non-array as the value itself, so
/// `$.k[1]` on a scalar wraps it in an array and `$.k[0]` replaces it; each
/// step's parent is checked to be the object or array the step needs first.
fn json_set(column: &str, path: &[JsonPathStep], value: &serde_json::Value) -> BoundSql {
    if path.is_empty() {
        return BoundSql::sql(column);
    }
    let guards = path.iter().enumerate().map(|(index, step)| {
        let kind = match step {
            JsonPathStep::Member(_) => Kind::Object,
            JsonPathStep::Element(_) => Kind::Array,
        };
        json::guard_sql(column, kind, json_path_text(&path[..index]))
    });
    bound!(
        "CASE WHEN ",
        BoundSql::join(guards, " AND "),
        format!(" THEN JSON_SET({column}, "),
        Value::String(Some(json_path_text(path))),
        ", JSON_EXTRACT(",
        json_text(value),
        format!(", '$')) ELSE {column} END")
    )
}

/// MariaDB's `LAG` and `LEAD` take no default. The same window finds no row
/// for a constant exactly where the default applies, where a `COALESCE` would
/// also replace a NULL value.
fn window_default(function: &str, column: &str, offset: &str, default: &str, over: &str) -> String {
    format!(
        "CASE WHEN {function}(1, {offset}) OVER ({over}) IS NULL THEN {default} \
         ELSE {function}({column}, {offset}) OVER ({over}) END"
    )
}

#[cfg(test)]
#[path = "../../../tests/unit/mysql_dialect_tests.rs"]
mod tests;
