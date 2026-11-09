use super::*;
use std::{
    hash::{Hash, Hasher},
    mem,
};

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::TinyInt(l), Self::TinyInt(r)) => l == r,
            (Self::SmallInt(l), Self::SmallInt(r)) => l == r,
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::BigInt(l), Self::BigInt(r)) => l == r,
            (Self::TinyUnsigned(l), Self::TinyUnsigned(r)) => l == r,
            (Self::SmallUnsigned(l), Self::SmallUnsigned(r)) => l == r,
            (Self::Unsigned(l), Self::Unsigned(r)) => l == r,
            (Self::BigUnsigned(l), Self::BigUnsigned(r)) => l == r,
            (Self::Float(l), Self::Float(r)) => cmp_f32(l, r),
            (Self::Double(l), Self::Double(r)) => cmp_f64(l, r),
            (Self::String(l), Self::String(r)) => l == r,
            (Self::Enum(l), Self::Enum(r)) => l == r,
            (Self::Char(l), Self::Char(r)) => l == r,
            (Self::Bytes(l), Self::Bytes(r)) => l == r,

            (Self::Json(l), Self::Json(r)) => cmp_json(l, r),

            (Self::ChronoDate(l), Self::ChronoDate(r)) => l == r,

            (Self::ChronoTime(l), Self::ChronoTime(r)) => l == r,

            (Self::ChronoDateTime(l), Self::ChronoDateTime(r)) => l == r,

            (Self::ChronoDateTimeUtc(l), Self::ChronoDateTimeUtc(r)) => l == r,

            (Self::ChronoDateTimeLocal(l), Self::ChronoDateTimeLocal(r)) => l == r,

            (Self::ChronoDateTimeWithTimeZone(l), Self::ChronoDateTimeWithTimeZone(r)) => l == r,

            (Self::Uuid(l), Self::Uuid(r)) => l == r,

            (Self::Decimal(l), Self::Decimal(r)) => l == r,
            (Self::Array(ty_l, values_l), Self::Array(ty_r, values_r)) => {
                ty_l == ty_r && values_l == values_r
            }

            _ => false,
        }
    }
}

impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            Value::Bool(v) => v.hash(state),
            Value::TinyInt(v) => v.hash(state),
            Value::SmallInt(v) => v.hash(state),
            Value::Int(v) => v.hash(state),
            Value::BigInt(v) => v.hash(state),
            Value::TinyUnsigned(v) => v.hash(state),
            Value::SmallUnsigned(v) => v.hash(state),
            Value::Unsigned(v) => v.hash(state),
            Value::BigUnsigned(v) => v.hash(state),
            Value::Float(v) => hash_f32(v, state),
            Value::Double(v) => hash_f64(v, state),
            Value::String(v) => v.hash(state),
            Value::Enum(v) => v.hash(state),
            Value::Char(v) => v.hash(state),
            Value::Bytes(v) => v.hash(state),

            Value::Json(value) => hash_json(value, state),

            Value::ChronoDate(naive_date) => naive_date.hash(state),

            Value::ChronoTime(naive_time) => naive_time.hash(state),

            Value::ChronoDateTime(naive_date_time) => naive_date_time.hash(state),

            Value::ChronoDateTimeUtc(date_time) => date_time.hash(state),

            Value::ChronoDateTimeLocal(date_time) => date_time.hash(state),

            Value::ChronoDateTimeWithTimeZone(date_time) => date_time.hash(state),

            Value::Uuid(uuid) => uuid.hash(state),

            Value::Decimal(decimal) => decimal.hash(state),
            Value::Array(array_type, vec) => {
                array_type.hash(state);
                vec.hash(state);
            }
        }
    }
}

/// The bits a float compares and hashes by: every NaN is one value, and so are
/// both zeroes. Any other two floats are equal exactly when their bits are.
fn float_key(v: f64) -> u64 {
    if v.is_nan() {
        f64::NAN.to_bits()
    } else if v == 0.0 {
        0
    } else {
        v.to_bits()
    }
}

fn hash_f32<H: Hasher>(v: &Option<f32>, state: &mut H) {
    hash_f64(&v.map(f64::from), state)
}

fn hash_f64<H: Hasher>(v: &Option<f64>, state: &mut H) {
    match v {
        Some(v) => float_key(*v).hash(state),
        None => "null".hash(state),
    }
}

fn cmp_f32(l: &Option<f32>, r: &Option<f32>) -> bool {
    cmp_f64(&l.map(f64::from), &r.map(f64::from))
}

fn cmp_f64(l: &Option<f64>, r: &Option<f64>) -> bool {
    l.map(float_key) == r.map(float_key)
}

fn hash_json<H: Hasher>(v: &Option<Box<Json>>, state: &mut H) {
    match v {
        Some(v) => serde_json::to_string(v).unwrap().hash(state),
        None => "null".hash(state),
    }
}

fn cmp_json(l: &Option<Box<Json>>, r: &Option<Box<Json>>) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => serde_json::to_string(l)
            .unwrap()
            .eq(&serde_json::to_string(r).unwrap()),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
#[path = "../../tests/unit/hashable_value_tests.rs"]
mod tests;
