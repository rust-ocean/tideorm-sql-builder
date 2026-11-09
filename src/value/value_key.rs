use super::Value;
use std::hash::{Hash, Hasher};

/// A [`Value`] compared the way a key column compares it.
///
/// Integers compare by value, whatever their width or signedness, so
/// `Int(1)`, `BigInt(1)` and `BigUnsigned(1)` are one key, and every other
/// value as [`Value`]'s own equality compares it. An integer and a
/// non-integer are never equal.
#[derive(Clone, Debug)]
pub struct ValueKey(Value);

impl ValueKey {
    /// The key of `value`.
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    /// The value the key was made from.
    pub fn value(&self) -> &Value {
        &self.0
    }

    /// The integer the value holds, widened, if it is one: `Some(None)` for
    /// an integer NULL.
    fn integer(&self) -> Option<Option<i128>> {
        Some(match self.0 {
            Value::TinyInt(value) => value.map(i128::from),
            Value::SmallInt(value) => value.map(i128::from),
            Value::Int(value) => value.map(i128::from),
            Value::BigInt(value) => value.map(i128::from),
            Value::TinyUnsigned(value) => value.map(i128::from),
            Value::SmallUnsigned(value) => value.map(i128::from),
            Value::Unsigned(value) => value.map(i128::from),
            Value::BigUnsigned(value) => value.map(i128::from),
            _ => return None,
        })
    }
}

impl From<Value> for ValueKey {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl PartialEq for ValueKey {
    fn eq(&self, other: &Self) -> bool {
        match (self.integer(), other.integer()) {
            (Some(left), Some(right)) => left == right,
            (None, None) => self.0 == other.0,
            _ => false,
        }
    }
}

impl Eq for ValueKey {}

impl Hash for ValueKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.integer() {
            Some(integer) => {
                true.hash(state);
                integer.hash(state);
            }
            None => {
                false.hash(state);
                self.0.hash(state);
            }
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/value_key_tests.rs"]
mod tests;
