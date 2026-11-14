//! Container for all SQL value types.

use std::borrow::Cow;

use crate::{ColumnKind, ColumnType, CommonSqlQueryBuilder, QueryBuilder, StringLen};

#[cfg(test)]
#[path = "../tests/unit/value_tests.rs"]
mod tests;

/// The crates whose types are values, and those types, re-exported so that callers name
/// the versions this crate is built with.
pub mod prelude {
    pub use serde_json::{self, Value as Json};

    pub use chrono::{
        self, DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc,
    };

    pub use rust_decimal::{self, Decimal};

    pub use uuid::{self, Uuid};
}
#[allow(unused_imports)]
use prelude::*;

mod binding_hash;
mod conversions;
mod hashable_value;

pub use conversions::*;
pub(crate) use conversions::{type_to_box_value, type_to_value};

mod value_class;
pub use value_class::*;

mod value_tuple;
pub use value_tuple::*;

mod value_key;
pub use value_key::ValueKey;

mod with_json;

pub use with_json::value_to_json_value;

mod with_chrono;

mod with_rust_decimal;

pub mod postgres_array;
mod with_uuid;

/// [`Value`] types variant for Postgres array
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ArrayType {
    Bool,
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    String,
    // box it because value size limit
    Enum(Box<EnumTypeName>),
    Char,
    Bytes,

    Json,

    ChronoDate,

    ChronoTime,

    ChronoDateTime,

    ChronoDateTimeUtc,

    ChronoDateTimeLocal,

    ChronoDateTimeWithTimeZone,

    Uuid,

    Decimal,
}

pub type EnumTypeName = std::sync::Arc<str>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Enum {
    pub type_name: EnumTypeName,
    pub value: Cow<'static, str>,
}

/// An enum value, or for a NULL the enum's type name, which
/// [`Value::array_type`] needs either way.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptionEnum {
    Some(Box<Enum>),
    None(EnumTypeName),
}

impl OptionEnum {
    /// Check if an enum value is set
    ///
    /// ```
    /// use sql_builder::{Enum, EnumTypeName, OptionEnum};
    ///
    /// let type_name: EnumTypeName = "Test".into();
    ///
    /// assert_eq!(false, OptionEnum::None(type_name.clone()).is_some());
    /// assert_eq!(
    ///     true,
    ///     OptionEnum::Some(Box::new(Enum {
    ///         type_name,
    ///         value: "Foo".into()
    ///     }))
    ///     .is_some()
    /// );
    /// ```
    pub fn is_some(&self) -> bool {
        matches!(*self, OptionEnum::Some(_))
    }
}

/// Value variants
///
/// We want the inner Value to be exactly 1 pointer sized, so anything larger should be boxed.
///
/// For hashing and equality, NaN == NaN, which contradicts Rust's built-in
/// implementation of NaN != NaN.
#[derive(Clone, Debug)]
pub enum Value {
    Bool(Option<bool>),
    TinyInt(Option<i8>),
    SmallInt(Option<i16>),
    Int(Option<i32>),
    BigInt(Option<i64>),
    TinyUnsigned(Option<u8>),
    SmallUnsigned(Option<u16>),
    Unsigned(Option<u32>),
    BigUnsigned(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    String(Option<String>),
    Enum(OptionEnum),
    Char(Option<char>),

    #[allow(clippy::box_collection)]
    Bytes(Option<Vec<u8>>),

    Json(Option<Box<Json>>),

    ChronoDate(Option<NaiveDate>),

    ChronoTime(Option<NaiveTime>),

    ChronoDateTime(Option<NaiveDateTime>),

    ChronoDateTimeUtc(Option<DateTime<Utc>>),

    ChronoDateTimeLocal(Option<DateTime<Local>>),

    ChronoDateTimeWithTimeZone(Option<DateTime<FixedOffset>>),

    Uuid(Option<Uuid>),

    Decimal(Option<Decimal>),
    Array(ArrayType, Option<Box<Vec<Value>>>),
}

/// This test is to check if the size of [`Value`] exceeds the limit.
///
/// If the size exceeds the limit, you should box the variant.
/// Previously, the size was 24. We bumped it to 32 such that `String`
/// can be unboxed.
///
/// This value should *never* be raised as this is critical to stability
/// in production, which can cause OOM or stack overflow for certain
/// high throughput workloads.
pub const VALUE_SIZE: usize = check_value_size();
const MAX_VALUE_SIZE: usize = 32;

const fn check_value_size() -> usize {
    if std::mem::size_of::<Value>() > MAX_VALUE_SIZE {
        panic!(
            "the size of Value shouldn't be greater than the expected MAX_VALUE_SIZE (32 bytes by default)"
        )
    }
    std::mem::size_of::<Value>()
}

impl Value {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }

    pub fn expect<T>(self, msg: &str) -> T
    where
        T: ValueType,
    {
        T::expect(self, msg)
    }

    /// Get the null variant of self
    ///
    /// ```
    /// use sql_builder::Value;
    ///
    /// let v = Value::Int(Some(2));
    /// let n = v.as_null();
    ///
    /// assert_eq!(n, Value::Int(None));
    ///
    /// // one liner:
    /// assert_eq!(Into::<Value>::into(2.2).as_null(), Value::Double(None));
    /// ```
    pub fn as_null(&self) -> Self {
        match self {
            Self::Bool(_) => Self::Bool(None),
            Self::TinyInt(_) => Self::TinyInt(None),
            Self::SmallInt(_) => Self::SmallInt(None),
            Self::Int(_) => Self::Int(None),
            Self::BigInt(_) => Self::BigInt(None),
            Self::TinyUnsigned(_) => Self::TinyUnsigned(None),
            Self::SmallUnsigned(_) => Self::SmallUnsigned(None),
            Self::Unsigned(_) => Self::Unsigned(None),
            Self::BigUnsigned(_) => Self::BigUnsigned(None),
            Self::Float(_) => Self::Float(None),
            Self::Double(_) => Self::Double(None),
            Self::String(_) => Self::String(None),
            Self::Enum(OptionEnum::Some(v)) => Self::Enum(OptionEnum::None(v.type_name.clone())),
            Self::Enum(OptionEnum::None(type_name)) => {
                Self::Enum(OptionEnum::None(type_name.clone()))
            }
            Self::Char(_) => Self::Char(None),
            Self::Bytes(_) => Self::Bytes(None),

            Self::Json(_) => Self::Json(None),

            Self::ChronoDate(_) => Self::ChronoDate(None),

            Self::ChronoTime(_) => Self::ChronoTime(None),

            Self::ChronoDateTime(_) => Self::ChronoDateTime(None),

            Self::ChronoDateTimeUtc(_) => Self::ChronoDateTimeUtc(None),

            Self::ChronoDateTimeLocal(_) => Self::ChronoDateTimeLocal(None),

            Self::ChronoDateTimeWithTimeZone(_) => Self::ChronoDateTimeWithTimeZone(None),

            Self::Uuid(_) => Self::Uuid(None),

            Self::Decimal(_) => Self::Decimal(None),
            Self::Array(ty, _) => Self::Array(ty.clone(), None),
        }
    }

    /// Check if a value is set
    ///
    /// ```
    /// use sql_builder::Value;
    ///
    /// assert_eq!(true, Value::Int(Some(2)).is_some());
    /// assert_eq!(false, Value::Int(None).is_some());
    /// ```
    pub fn is_some(&self) -> bool {
        match self {
            Self::Bool(v) => v.is_some(),
            Self::TinyInt(v) => v.is_some(),
            Self::SmallInt(v) => v.is_some(),
            Self::Int(v) => v.is_some(),
            Self::BigInt(v) => v.is_some(),
            Self::TinyUnsigned(v) => v.is_some(),
            Self::SmallUnsigned(v) => v.is_some(),
            Self::Unsigned(v) => v.is_some(),
            Self::BigUnsigned(v) => v.is_some(),
            Self::Float(v) => v.is_some(),
            Self::Double(v) => v.is_some(),
            Self::String(v) => v.is_some(),
            Self::Enum(v) => v.is_some(),
            Self::Char(v) => v.is_some(),
            Self::Bytes(v) => v.is_some(),

            Self::Json(v) => v.is_some(),

            Self::ChronoDate(v) => v.is_some(),

            Self::ChronoTime(v) => v.is_some(),

            Self::ChronoDateTime(v) => v.is_some(),

            Self::ChronoDateTimeUtc(v) => v.is_some(),

            Self::ChronoDateTimeLocal(v) => v.is_some(),

            Self::ChronoDateTimeWithTimeZone(v) => v.is_some(),

            Self::Uuid(v) => v.is_some(),

            Self::Decimal(v) => v.is_some(),
            Self::Array(_, v) => v.is_some(),
        }
    }

    /// Get a default value of self's type
    ///
    /// ```
    /// use sql_builder::Value;
    ///
    /// let v = Value::Int(None);
    /// let n = v.dummy_value();
    /// assert_eq!(n, Value::Int(Some(0)));
    /// ```
    pub fn dummy_value(&self) -> Self {
        match self {
            Self::Bool(_) => Self::Bool(Some(Default::default())),
            Self::TinyInt(_) => Self::TinyInt(Some(Default::default())),
            Self::SmallInt(_) => Self::SmallInt(Some(Default::default())),
            Self::Int(_) => Self::Int(Some(Default::default())),
            Self::BigInt(_) => Self::BigInt(Some(Default::default())),
            Self::TinyUnsigned(_) => Self::TinyUnsigned(Some(Default::default())),
            Self::SmallUnsigned(_) => Self::SmallUnsigned(Some(Default::default())),
            Self::Unsigned(_) => Self::Unsigned(Some(Default::default())),
            Self::BigUnsigned(_) => Self::BigUnsigned(Some(Default::default())),
            Self::Float(_) => Self::Float(Some(Default::default())),
            Self::Double(_) => Self::Double(Some(Default::default())),
            Self::String(_) => Self::String(Some(Default::default())),
            Self::Enum(v) => {
                let type_name = match v {
                    OptionEnum::Some(v) => v.type_name.clone(),
                    OptionEnum::None(type_name) => type_name.clone(),
                };
                Self::Enum(OptionEnum::Some(Box::new(Enum {
                    type_name,
                    value: Cow::Borrowed(""),
                })))
            }
            Self::Char(_) => Self::Char(Some(Default::default())),
            Self::Bytes(_) => Self::Bytes(Some(Default::default())),

            Self::Json(_) => Self::Json(Some(Default::default())),

            Self::ChronoDate(_) => Self::ChronoDate(Some(Default::default())),

            Self::ChronoTime(_) => Self::ChronoTime(Some(Default::default())),

            Self::ChronoDateTime(_) => Self::ChronoDateTime(Some(Default::default())),

            Self::ChronoDateTimeUtc(_) => Self::ChronoDateTimeUtc(Some(Default::default())),

            Self::ChronoDateTimeLocal(_) => Self::ChronoDateTimeLocal(Some(Default::default())),

            Self::ChronoDateTimeWithTimeZone(_) => {
                Self::ChronoDateTimeWithTimeZone(Some(Default::default()))
            }

            Self::Uuid(_) => Self::Uuid(Some(Default::default())),

            Self::Decimal(_) => Self::Decimal(Some(Default::default())),
            Self::Array(ty, _) => Self::Array(ty.clone(), Some(Default::default())),
        }
    }

    pub fn array_type(&self) -> ArrayType {
        #[allow(unused_imports)]
        use std::ops::Deref;

        fn array_type_of<V: ValueType>(_: &Option<V>) -> ArrayType {
            V::array_type()
        }

        #[allow(dead_code)]
        fn array_type_of_ref<V: ValueType>(_: Option<&V>) -> ArrayType {
            V::array_type()
        }

        match self {
            Self::Bool(v) => array_type_of(v),
            Self::TinyInt(v) => array_type_of(v),
            Self::SmallInt(v) => array_type_of(v),
            Self::Int(v) => array_type_of(v),
            Self::BigInt(v) => array_type_of(v),
            Self::TinyUnsigned(v) => array_type_of(v),
            Self::SmallUnsigned(v) => array_type_of(v),
            Self::Unsigned(v) => array_type_of(v),
            Self::BigUnsigned(v) => array_type_of(v),
            Self::Float(v) => array_type_of(v),
            Self::Double(v) => array_type_of(v),
            Self::String(v) => array_type_of(v),

            Self::Enum(v) => ArrayType::Enum(Box::new(match v {
                OptionEnum::Some(v) => v.type_name.clone(),
                OptionEnum::None(type_name) => type_name.clone(),
            })),

            Self::Char(v) => array_type_of(v),
            Self::Bytes(v) => array_type_of(v),

            Self::Json(v) => array_type_of_ref(v.as_ref().map(|v| v.deref())),

            Self::ChronoDate(v) => array_type_of(v),

            Self::ChronoTime(v) => array_type_of(v),

            Self::ChronoDateTime(v) => array_type_of(v),

            Self::ChronoDateTimeUtc(v) => array_type_of(v),

            Self::ChronoDateTimeLocal(v) => array_type_of(v),

            Self::ChronoDateTimeWithTimeZone(v) => array_type_of(v),

            Self::Uuid(v) => array_type_of(v),

            Self::Decimal(v) => array_type_of(v),
            Self::Array(v, _) => v.clone(),
        }
    }
}
