use super::Value;
use chrono::Offset;
use std::hash::{Hash, Hasher};

impl Value {
    /// Hash the binding representation, including details ignored by value equality.
    ///
    /// This is deliberately separate from value hashing: numerically equal decimals,
    /// float zeroes and timestamps can carry different data to a driver.
    pub fn hash_binding<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Float(value) => value.map(f32::to_bits).hash(state),
            Self::Double(value) => value.map(f64::to_bits).hash(state),
            Self::Decimal(value) => value.map(|v| v.serialize()).hash(state),
            Self::ChronoDateTimeUtc(value) => value
                .map(|v| (v.naive_utc(), v.offset().fix().local_minus_utc()))
                .hash(state),
            Self::ChronoDateTimeLocal(value) => value
                .map(|v| (v.naive_utc(), v.offset().fix().local_minus_utc()))
                .hash(state),
            Self::ChronoDateTimeWithTimeZone(value) => value
                .map(|v| (v.naive_utc(), v.offset().local_minus_utc()))
                .hash(state),
            // A JSON `null` document is a value, which SQL NULL is not.
            Self::Json(value) => {
                value.is_some().hash(state);
                self.hash(state);
            }
            Self::Array(element_type, values) => {
                element_type.hash(state);
                values.is_some().hash(state);
                if let Some(values) = values {
                    values.len().hash(state);
                    for value in values.iter() {
                        value.hash_binding(state);
                    }
                }
            }
            // These variants already hash every part of their binding payload,
            // including the enum type name and the distinction between NULL and empty.
            _ => self.hash(state),
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/binding_hash_tests.rs"]
mod tests;
