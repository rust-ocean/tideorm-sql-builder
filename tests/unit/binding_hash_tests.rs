use super::*;
use crate::{Enum, OptionEnum};
use rust_decimal::Decimal;
use std::collections::hash_map::DefaultHasher;

fn key(value: &Value) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash_binding(&mut hasher);
    hasher.finish()
}

#[test]
fn binding_hash_preserves_decimal_scale_and_timestamp_offset() {
    let a = Value::from(Decimal::new(10, 1));
    let b = Value::from(Decimal::new(100, 2));
    assert_eq!(a, b);
    assert_ne!(key(&a), key(&b));
    let a = Value::from(chrono::DateTime::parse_from_rfc3339("2026-11-01T12:00:00+00:00").unwrap());
    let b = Value::from(chrono::DateTime::parse_from_rfc3339("2026-11-01T14:00:00+02:00").unwrap());
    assert_eq!(a, b);
    assert_ne!(key(&a), key(&b));
    assert_eq!(key(&a), key(&a.clone()));
}

#[test]
fn binding_hash_preserves_null_types_float_bits_and_enum_names() {
    assert_ne!(key(&Value::Int(None)), key(&Value::BigInt(None)));
    assert_ne!(
        key(&Value::Double(Some(0.0))),
        key(&Value::Double(Some(-0.0)))
    );
    assert_ne!(
        key(&Value::Float(Some(f32::from_bits(0x7fc00001)))),
        key(&Value::Float(Some(f32::from_bits(0x7fc00002))))
    );
    let enum_value = |name: &str| {
        Value::Enum(OptionEnum::Some(Box::new(Enum {
            type_name: name.into(),
            value: "active".into(),
        })))
    };
    assert_ne!(key(&enum_value("a")), key(&enum_value("b")));
    for value in [
        Value::Int(None),
        Value::from("hello"),
        enum_value("a"),
        Value::from(Decimal::new(10, 1)),
    ] {
        assert_eq!(key(&value), key(&value.clone()));
    }
}
#[test]
fn binding_hash_preserves_array_types_nulls_and_element_representations() {
    use crate::ArrayType;
    let array = |ty, values| Value::Array(ty, Some(Box::new(values)));
    assert_ne!(
        key(&array(ArrayType::Int, vec![])),
        key(&array(ArrayType::BigInt, vec![]))
    );
    assert_ne!(
        key(&Value::Array(ArrayType::Int, None)),
        key(&array(ArrayType::Int, vec![]))
    );
    let a = array(ArrayType::Decimal, vec![Decimal::new(10, 1).into()]);
    let b = array(ArrayType::Decimal, vec![Decimal::new(100, 2).into()]);
    assert_eq!(a, b);
    assert_ne!(key(&a), key(&b));
    assert_eq!(key(&a), key(&a.clone()));
}
