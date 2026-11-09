//! PostgreSQL's `@>` and `<@` on MySQL and MariaDB.
//!
//! `JSON_CONTAINS` differs from PostgreSQL in one place: a candidate that is
//! not an array is contained in an array holding it at any depth, so
//! `{"tags": "a"}` is contained in `{"tags": ["a", "b"]}`. The document on the
//! parameter side is known when the statement is built, so each of its object
//! paths adds a check that the column's value there has the kind PostgreSQL
//! requires. Below an array the paths are not known, so everything inside an
//! array keeps MySQL's reading: `[{"k": 1}]` is contained in `[{"k": [1, 2]}]`.
//!
//! The document is bound as JSON text, which `JSON_CONTAINS` parses on MySQL
//! and MariaDB alike; MariaDB has no `CAST(.. AS JSON)`.

use crate::Value;
use crate::backend::bound_sql::bound;
use crate::backend::dialect::{BoundSql, json_text};
use crate::backend::json_path::member_path;

/// `column @> candidate`.
pub(super) fn contains(column: &str, candidate: &serde_json::Value) -> BoundSql {
    let mut guards = Vec::new();
    contains_guards(candidate, "$".to_string(), &mut guards);
    render(
        bound!(
            format!("JSON_CONTAINS({column}, "),
            json_text(candidate),
            ")"
        ),
        column,
        guards,
    )
}

/// `column <@ target`.
pub(super) fn contained_by(column: &str, target: &serde_json::Value) -> BoundSql {
    let mut guards = Vec::new();
    contained_by_guards(target, "$".to_string(), &mut guards);
    render(
        bound!("JSON_CONTAINS(", json_text(target), format!(", {column})")),
        column,
        guards,
    )
}

/// What the column's value at a path must be.
#[derive(Clone, Copy)]
pub(super) enum Kind {
    Object,
    Array,
    NotArray,
    NotObject,
    ArrayOrAbsent,
}

/// The test that the column's value at `path` is `kind`.
pub(super) fn guard_sql(column: &str, kind: Kind, path: String) -> BoundSql {
    let value_type = bound!(
        format!("JSON_TYPE(JSON_EXTRACT({column}, "),
        Value::String(Some(path)),
        "))"
    );
    match kind {
        Kind::Object => bound!(value_type, " = 'OBJECT'"),
        Kind::Array => bound!(value_type, " = 'ARRAY'"),
        Kind::NotArray => bound!(value_type, " <> 'ARRAY'"),
        Kind::NotObject => bound!(value_type, " <> 'OBJECT'"),
        Kind::ArrayOrAbsent => bound!("COALESCE(", value_type, ", 'ARRAY') = 'ARRAY'"),
    }
}

/// A candidate object only matches an object, and a scalar below the top level
/// never matches an array; only a top-level scalar may match a top-level array.
fn contains_guards(candidate: &serde_json::Value, path: String, guards: &mut Vec<(String, Kind)>) {
    match candidate {
        serde_json::Value::Object(members) => {
            for (key, value) in members {
                contains_guards(value, member_path(&path, key), guards);
            }
            guards.push((path, Kind::Object));
        }
        serde_json::Value::Array(_) => {}
        _ if path == "$" => {}
        _ => guards.push((path, Kind::NotArray)),
    }
}

/// Where the target holds an array below the top level, the column's value
/// there, if it has one, must be an array; at the top level it must not be an
/// object, since only a scalar may be contained in a top-level array.
fn contained_by_guards(target: &serde_json::Value, path: String, guards: &mut Vec<(String, Kind)>) {
    match target {
        serde_json::Value::Object(members) => {
            for (key, value) in members {
                contained_by_guards(value, member_path(&path, key), guards);
            }
        }
        serde_json::Value::Array(_) if path == "$" => guards.push((path, Kind::NotObject)),
        serde_json::Value::Array(_) => guards.push((path, Kind::ArrayOrAbsent)),
        _ => {}
    }
}

/// `containment AND` each guard.
fn render(mut containment: BoundSql, column: &str, guards: Vec<(String, Kind)>) -> BoundSql {
    for (path, kind) in guards {
        containment
            .push(" AND ")
            .push(guard_sql(column, kind, path));
    }
    containment
}
