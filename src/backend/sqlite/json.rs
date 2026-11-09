//! JSON containment and equality for SQLite, which has no `@>` or `<@`
//! operator and compares JSON only as text.
//!
//! PostgreSQL's reading is rebuilt from `json_each` rows: an object candidate
//! needs every member contained under the same key, an array candidate needs
//! every element contained by some element, and a scalar has to equal the
//! value — or, at the top level only, be an element of an array. Equality
//! needs as many members or elements as the value, each equal to the value's
//! own. Keys and scalars are bound as parameters in the order their `?`
//! appear.

use crate::Value;
use crate::backend::bound_sql::{BoundSql, bound};
use crate::backend::dialect::joined;

/// One JSON node the test inspects: the column itself, or a `json_each` row.
struct Node {
    /// The node's JSON text.
    json: String,
    /// Its type as `json_type` and `json_each.type` spell it.
    kind: String,
    raw_json: String,
    /// Its value as an SQL scalar.
    atom: String,
}

impl Node {
    fn document(column_sql: &str) -> Self {
        Self {
            json: column_sql.to_string(),
            raw_json: format!("({column_sql} -> '$')"),
            kind: format!("json_type({column_sql})"),
            atom: format!("json_extract({column_sql}, '$')"),
        }
    }

    fn row(alias: &str) -> Self {
        Self {
            json: format!("{alias}.value"),
            raw_json: format!("({alias}.json -> {alias}.fullkey)"),
            kind: format!("{alias}.type"),
            atom: format!("{alias}.atom"),
        }
    }
}

/// SQL testing that the JSON document in `column_sql` contains `candidate`.
pub(crate) fn sqlite_json_contains(column_sql: &str, candidate: &serde_json::Value) -> BoundSql {
    JsonTests::default().contains(&Node::document(column_sql), candidate, true)
}

/// SQL testing that the JSON document in `column_sql` is contained by
/// `container`: the same test with the known document on the other side.
pub(crate) fn sqlite_json_contained_by(
    column_sql: &str,
    container: &serde_json::Value,
) -> BoundSql {
    JsonTests::default().contained_by(&Node::document(column_sql), container, true)
}

/// SQL testing that the JSON document in `column_sql` equals `value`.
pub(crate) fn sqlite_json_equals(column_sql: &str, value: &serde_json::Value) -> BoundSql {
    JsonTests::default().equals(&Node::document(column_sql), value)
}

/// SQL testing that the `json_each` row `alias` holds `value`.
pub(crate) fn sqlite_json_element_equals(alias: &str, value: &serde_json::Value) -> BoundSql {
    JsonTests::default().equals(&Node::row(alias), value)
}

/// Numbers the `json_each` aliases of one test.
#[derive(Default)]
struct JsonTests {
    aliases: usize,
}

impl JsonTests {
    fn alias(&mut self) -> String {
        self.aliases += 1;
        format!("tide_json_{}", self.aliases)
    }

    fn contains(
        &mut self,
        node: &Node,
        candidate: &serde_json::Value,
        top_level: bool,
    ) -> BoundSql {
        use serde_json::Value as Json;

        match candidate {
            Json::Object(members) => {
                let mut checks = vec![BoundSql::sql(format!("{} = 'object'", node.kind))];
                for (key, value) in members {
                    let alias = self.alias();
                    let member = self.contains(&Node::row(&alias), value, false);
                    checks.push(bound!(
                        format!(
                            "EXISTS (SELECT 1 FROM json_each({}) AS {alias} WHERE {alias}.key = ",
                            node.json
                        ),
                        Value::from(key.clone()),
                        " AND ",
                        member,
                        ")"
                    ));
                }
                all_of(checks)
            }
            Json::Array(elements) => {
                let mut checks = vec![BoundSql::sql(format!("{} = 'array'", node.kind))];
                for element in elements {
                    checks.push(self.some_element(node, element));
                }
                all_of(checks)
            }
            scalar if top_level => {
                let equal = self.scalar_equals(node, scalar);
                let element = self.some_element(node, scalar);
                bound!(
                    "(",
                    equal,
                    format!(" OR ({} = 'array' AND ", node.kind),
                    element,
                    "))"
                )
            }
            scalar => self.scalar_equals(node, scalar),
        }
    }

    /// `container` contains `node`: every member of an object node sits under
    /// a key the container has and is contained by its value there, every
    /// element of an array node is contained by some element of the container,
    /// and a scalar node equals the container, or, at the top level only, an
    /// element of it.
    fn contained_by(
        &mut self,
        node: &Node,
        container: &serde_json::Value,
        top_level: bool,
    ) -> BoundSql {
        use serde_json::Value as Json;

        match container {
            Json::Object(members) => {
                let alias = self.alias();
                let row = Node::row(&alias);
                let mut placed = Vec::new();
                for (key, value) in members {
                    let member = self.contained_by(&row, value, false);
                    placed.push(bound!(
                        format!("({alias}.key = "),
                        Value::from(key.clone()),
                        " AND ",
                        member,
                        ")"
                    ));
                }
                none_outside(node, &alias, "object", placed)
            }
            Json::Array(elements) => {
                let alias = self.alias();
                let row = Node::row(&alias);
                let placed: Vec<BoundSql> = elements
                    .iter()
                    .map(|element| self.contained_by(&row, element, false))
                    .collect();
                let array = none_outside(node, &alias, "array", placed);
                if !top_level {
                    return array;
                }
                let scalars: Vec<BoundSql> = elements
                    .iter()
                    .filter(|element| !element.is_array() && !element.is_object())
                    .map(|element| self.scalar_equals(node, element))
                    .collect();
                bound!("(", array, " OR ", any_of(scalars), ")")
            }
            scalar => self.scalar_equals(node, scalar),
        }
    }

    /// `node` holds exactly `value`, compared as JSON: an object has the
    /// same keys, in any order, holding equal values, an array the same
    /// elements in the same order, and a number the same value however it
    /// is written (`1` and `1.0`).
    fn equals(&mut self, node: &Node, value: &serde_json::Value) -> BoundSql {
        use serde_json::Value as Json;

        match value {
            Json::Object(members) => {
                let mut checks = vec![
                    BoundSql::sql(format!("{} = 'object'", node.kind)),
                    BoundSql::sql(format!(
                        "(SELECT COUNT(*) FROM json_each({})) = {}",
                        node.json,
                        members.len()
                    )),
                ];
                for (key, member) in members {
                    let alias = self.alias();
                    let equal = self.equals(&Node::row(&alias), member);
                    checks.push(bound!(
                        format!(
                            "EXISTS (SELECT 1 FROM json_each({}) AS {alias} WHERE {alias}.key = ",
                            node.json
                        ),
                        Value::from(key.clone()),
                        " AND ",
                        equal,
                        ")"
                    ));
                }
                all_of(checks)
            }
            Json::Array(elements) => {
                let mut checks = vec![
                    BoundSql::sql(format!("{} = 'array'", node.kind)),
                    BoundSql::sql(format!(
                        "json_array_length({}) = {}",
                        node.json,
                        elements.len()
                    )),
                ];
                for (index, element) in elements.iter().enumerate() {
                    let alias = self.alias();
                    let equal = self.equals(&Node::row(&alias), element);
                    checks.push(bound!(
                        format!(
                            "EXISTS (SELECT 1 FROM json_each({}) AS {alias} WHERE {alias}.key = {index} AND ",
                            node.json
                        ),
                        equal,
                        ")"
                    ));
                }
                all_of(checks)
            }
            scalar => self.scalar_equals(node, scalar),
        }
    }

    /// Some element of the array `node` contains `candidate`.
    fn some_element(&mut self, node: &Node, candidate: &serde_json::Value) -> BoundSql {
        let alias = self.alias();
        let element = self.contains(&Node::row(&alias), candidate, false);
        bound!(
            format!(
                "EXISTS (SELECT 1 FROM json_each({}) AS {alias} WHERE ",
                node.json
            ),
            element,
            ")"
        )
    }

    /// The node holds the number whose JSON text is `text`.
    fn number_equals(node: &Node, text: &str) -> BoundSql {
        // Compare decimal tokens, not SQLite's already-rounded REAL.
        // Normalizing coefficient and exponent also makes 1, 1.0 and
        // 10e-1 equal without conflating adjacent large decimal values.
        let (negative, coefficient, scale) = decimal_parts(text);
        let comparison = match (coefficient.is_empty(), scale) {
            (true, _) => BoundSql::sql("coefficient = ''"),
            (false, Some(scale)) => bound!(
                "coefficient = ",
                Value::from(coefficient),
                " AND negative = ",
                Value::from(negative),
                " AND scale = ",
                Value::from(scale)
            ),
            // SQLite reads no exponent an `i64` cannot hold, so no
            // stored number has that scale.
            (false, None) => BoundSql::sql("0"),
        };
        bound!(
            format!(
                "({kind} IN ('integer', 'real') AND (WITH \
                 tide_num_token AS (SELECT lower({raw}) AS token), \
                 tide_num_unsigned AS (SELECT substr(token, 1, 1) = '-' AS negative, \
                    CASE WHEN substr(token, 1, 1) = '-' THEN substr(token, 2) ELSE token END AS digits FROM tide_num_token), \
                 tide_num_parts AS (SELECT negative, \
                    CASE WHEN instr(digits, 'e') > 0 THEN substr(digits, 1, instr(digits, 'e') - 1) ELSE digits END AS mantissa, \
                    CASE WHEN instr(digits, 'e') > 0 THEN CAST(substr(digits, instr(digits, 'e') + 1) AS INTEGER) ELSE 0 END AS exponent FROM tide_num_unsigned), \
                 tide_num_digits AS (SELECT negative, replace(mantissa, '.', '') AS digits, \
                    exponent - CASE WHEN instr(mantissa, '.') > 0 THEN length(mantissa) - instr(mantissa, '.') ELSE 0 END AS exponent FROM tide_num_parts), \
                 tide_num_normal AS (SELECT negative, ltrim(rtrim(digits, '0'), '0') AS coefficient, \
                    exponent + length(digits) - length(rtrim(digits, '0')) AS scale FROM tide_num_digits) \
                 SELECT ",
                kind = node.kind,
                raw = node.raw_json,
            ),
            comparison,
            " FROM tide_num_normal))"
        )
    }

    fn scalar_equals(&self, node: &Node, scalar: &serde_json::Value) -> BoundSql {
        use serde_json::Value as Json;

        match scalar {
            Json::Null => BoundSql::sql(format!("{} = 'null'", node.kind)),
            Json::Bool(true) => BoundSql::sql(format!("{} = 'true'", node.kind)),
            Json::Bool(false) => BoundSql::sql(format!("{} = 'false'", node.kind)),
            Json::Number(number) => Self::number_equals(node, &number.to_string()),
            Json::String(text) => bound!(
                format!("({} = 'text' AND {} = ", node.kind, node.atom),
                Value::from(text.clone()),
                ")"
            ),
            Json::Array(_) | Json::Object(_) => unreachable!("containers are not scalars"),
        }
    }
}

/// `node` is of `kind` and has no `json_each` row (`alias`) outside the
/// `placed` alternatives.
fn none_outside(node: &Node, alias: &str, kind: &str, placed: Vec<BoundSql>) -> BoundSql {
    bound!(
        format!(
            "({} = '{kind}' AND NOT EXISTS (SELECT 1 FROM json_each({}) AS {alias} WHERE NOT ",
            node.kind, node.json
        ),
        any_of(placed),
        "))"
    )
}

/// `checks` joined with AND, parenthesized.
fn all_of(checks: Vec<BoundSql>) -> BoundSql {
    joined(checks, " AND ", "(1)")
}

/// `alternatives` joined with OR, parenthesized; an empty list is false.
fn any_of(alternatives: Vec<BoundSql>) -> BoundSql {
    joined(alternatives, " OR ", "(0)")
}

/// A JSON number's text as its sign, its coefficient without leading or
/// trailing zeros (empty for zero) and the power of ten that multiplies it,
/// `None` when no `i64` holds that power: a number whose text keeps its
/// source digits (serde_json's `arbitrary_precision`) can carry any exponent.
fn decimal_parts(text: &str) -> (bool, String, Option<i64>) {
    let negative = text.starts_with('-');
    let unsigned = text.trim_start_matches('-');
    let (mantissa, exponent) = match unsigned.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, exponent.parse::<i64>().ok()),
        None => (unsigned, Some(0)),
    };
    let fraction = mantissa
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let digits = mantissa.replace('.', "");
    let trimmed = digits.trim_end_matches('0');
    let coefficient = trimmed.trim_start_matches('0').to_string();
    let scale = exponent.and_then(|exponent| {
        exponent
            .checked_sub(i64::try_from(fraction).ok()?)?
            .checked_add(i64::try_from(digits.len() - trimmed.len()).ok()?)
    });
    (negative, coefficient, scale)
}

#[cfg(test)]
#[path = "../../../tests/unit/sqlite_json_tests.rs"]
mod tests;
