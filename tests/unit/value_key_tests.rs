use crate::{Value, ValueKey};
use std::collections::HashSet;

fn key(value: impl Into<Value>) -> ValueKey {
    ValueKey::new(value.into())
}

#[test]
fn integers_are_one_key_whatever_their_width_or_sign() {
    let ones = [
        key(1i8),
        key(1i16),
        key(1i32),
        key(1i64),
        key(1u8),
        key(1u16),
        key(1u32),
        key(1u64),
    ];
    let distinct: HashSet<ValueKey> = ones.iter().cloned().collect();
    assert_eq!(distinct.len(), 1);
    assert_eq!(key(-1i8), key(-1i64));
    assert_ne!(key(-1i64), key(u64::MAX));
    assert_eq!(key(u64::MAX), key(u64::MAX));
    assert_eq!(key(Value::Int(None)), key(Value::BigUnsigned(None)));
}

#[test]
fn other_values_compare_as_values_do() {
    assert_ne!(key(1i32), key("1"));
    assert_ne!(key(1i32), key(1.0f64));
    assert_ne!(key(1i32), key(true));
    assert_eq!(key("a"), key("a".to_owned()));
    assert_ne!(key("a"), key("A"));
    assert_eq!(key(1.5f64), key(1.5f64));
    let keys: HashSet<ValueKey> = [key("1"), key(1i64), key(1i32), key("1")]
        .into_iter()
        .collect();
    assert_eq!(keys.len(), 2);
}

#[test]
fn a_key_keeps_its_value() {
    assert_eq!(key(7i16).value(), &Value::SmallInt(Some(7)));
    assert_ne!(key(Value::BigInt(None)), key(Value::String(None)));
}
