use super::*;
use std::sync::Arc;

// `Vec<T>` converts to an array for every `T` but `u8`, whose `Vec` converts to
// bytes; without negative trait bounds, the other element types are listed.
pub trait NotU8 {}

impl NotU8 for bool {}
impl NotU8 for i8 {}
impl NotU8 for i16 {}
impl NotU8 for i32 {}
impl NotU8 for i64 {}
impl NotU8 for u16 {}
impl NotU8 for u32 {}
impl NotU8 for u64 {}
impl NotU8 for f32 {}
impl NotU8 for f64 {}
impl NotU8 for char {}
impl NotU8 for String {}
impl NotU8 for Vec<u8> {}

impl NotU8 for Json {}

impl NotU8 for NaiveDate {}

impl NotU8 for NaiveTime {}

impl NotU8 for NaiveDateTime {}

impl<Tz> NotU8 for DateTime<Tz> where Tz: chrono::TimeZone {}

impl NotU8 for Decimal {}

impl NotU8 for Uuid {}

impl NotU8 for uuid::fmt::Braced {}

impl NotU8 for uuid::fmt::Hyphenated {}

impl NotU8 for uuid::fmt::Simple {}

impl NotU8 for uuid::fmt::Urn {}

/// If this blanket impl is causing trouble, please open an issue
impl<T> NotU8 for &T where T: NotU8 {}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value> + NotU8 + ValueType,
{
    fn from(x: Vec<T>) -> Value {
        Value::Array(
            T::array_type(),
            Some(Box::new(x.into_iter().map(|e| e.into()).collect())),
        )
    }
}

impl<T> Nullable for Vec<T>
where
    T: Into<Value> + NotU8 + ValueType,
{
    fn null() -> Value {
        Value::Array(T::array_type(), None)
    }
}

impl<T> ValueType for Vec<T>
where
    T: NotU8 + ValueType,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Array(ty, Some(v)) if T::array_type() == ty => {
                v.into_iter().map(try_array_element::<T>).collect()
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Vec<T>).to_owned()
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        use ColumnType::*;
        Array(Arc::new(T::column_type()))
    }
}

impl<T> ValueType for Vec<Option<T>>
where
    T: NotU8 + ValueType,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Array(ty, Some(v)) if T::array_type() == ty => v
                .into_iter()
                .map(|e| {
                    if e.is_some() {
                        T::try_from(e).map(Some)
                    } else {
                        Ok(None)
                    }
                })
                .collect(),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Vec<Option<T>>).to_owned()
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        use ColumnType::*;
        Array(Arc::new(T::column_type()))
    }
}

impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    pub fn as_ref_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(_, v) => v.as_ref().map(|v| v.as_ref()),
            _ => panic!("not Value::Array"),
        }
    }
}

/// Convert a non-NULL array element without accepting a wrong variant.
pub fn try_array_element<T: ValueType>(value: Value) -> Result<T, ValueTypeErr> {
    if value.is_some() {
        T::try_from(value)
    } else {
        Err(ValueTypeErr)
    }
}

/// Borrow a non-NULL enum array element, validating its representation.
pub fn enum_array_element(value: &Value) -> Result<&Enum, ValueTypeErr> {
    match value {
        Value::Enum(OptionEnum::Some(value)) => Ok(value),
        _ => Err(ValueTypeErr),
    }
}
