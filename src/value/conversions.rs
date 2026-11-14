use super::*;

impl From<&[u8]> for Value {
    fn from(x: &[u8]) -> Value {
        Value::Bytes(Some(x.into()))
    }
}

impl From<&str> for Value {
    fn from(x: &str) -> Value {
        Value::String(Some(x.to_owned()))
    }
}

impl From<&String> for Value {
    fn from(x: &String) -> Value {
        Value::String(Some(x.clone()))
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value> + Nullable,
{
    fn from(x: Option<T>) -> Value {
        match x {
            Some(v) => v.into(),
            None => T::null(),
        }
    }
}

impl From<Cow<'_, str>> for Value {
    fn from(x: Cow<'_, str>) -> Value {
        x.into_owned().into()
    }
}

impl From<Enum> for Value {
    fn from(value: Enum) -> Value {
        Value::Enum(OptionEnum::Some(Box::new(value)))
    }
}

impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sql = String::new();
        CommonSqlQueryBuilder
            .write_value(&mut sql, self)
            .map_err(|_| std::fmt::Error)?;
        f.write_str(&sql)
    }
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a type a column can hold",
    label = "not a column type",
    note = "a column's type implements `ValueType`: derive `DeriveValueType` for a newtype or `DeriveActiveEnum` for an enum, or store the value as a `String`"
)]
pub trait ValueType: Sized {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr>;

    fn unwrap(v: Value) -> Self {
        Self::try_from(v).unwrap()
    }

    fn expect(v: Value, msg: &str) -> Self {
        Self::try_from(v).expect(msg)
    }

    fn is_option() -> bool {
        false
    }

    fn type_name() -> String;

    fn array_type() -> ArrayType;

    fn column_type() -> ColumnType;

    /// The [`kind`](ColumnType::kind) of [`column_type`](Self::column_type),
    /// readable in a `const`; [`ColumnKind::Other`] unless the type sets it.
    const COLUMN_KIND: ColumnKind = ColumnKind::Other;

    fn enum_type_name() -> Option<&'static str> {
        None
    }
}

impl<T> ValueType for Option<T>
where
    T: ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if v == T::null() {
            Ok(None)
        } else {
            Ok(Some(T::try_from(v)?))
        }
    }

    fn is_option() -> bool {
        true
    }

    fn type_name() -> String {
        format!("Option<{}>", T::type_name())
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }

    const COLUMN_KIND: ColumnKind = T::COLUMN_KIND;
}

impl ValueType for Cow<'_, str> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(x)) => Ok((x).into()),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Cow<str>".into()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::Text
    }

    const COLUMN_KIND: ColumnKind = ColumnKind::Text;
}

#[derive(Debug)]
pub struct ValueTypeErr;

impl std::error::Error for ValueTypeErr {}

impl std::fmt::Display for ValueTypeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Value type mismatch")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

pub trait Nullable {
    fn null() -> Value;
}

impl Nullable for &str {
    fn null() -> Value {
        Value::String(None)
    }
}

impl Nullable for Enum {
    fn null() -> Value {
        Value::Enum(OptionEnum::None("".into()))
    }
}

impl ValueType for Enum {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Enum(OptionEnum::Some(v)) => Ok(*v),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Enum".into()
    }

    // These implementations probably won’t actually be used, so there
    // shouldn’t cause any runtime issues.
    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }

    const COLUMN_KIND: ColumnKind = ColumnKind::Text;
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(x))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::$name
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }

            const COLUMN_KIND: ColumnKind = {
                use ColumnType::*;
                $col_type
            }
            .kind();
        }
    };
}

#[allow(unused_imports)]
pub(crate) use type_to_value;

type_to_value!(bool, Bool, Boolean);
type_to_value!(i8, TinyInt, TinyInteger);
type_to_value!(i16, SmallInt, SmallInteger);
type_to_value!(i32, Int, Integer);
type_to_value!(i64, BigInt, BigInteger);
type_to_value!(u8, TinyUnsigned, TinyUnsigned);
type_to_value!(u16, SmallUnsigned, SmallUnsigned);
type_to_value!(u32, Unsigned, Unsigned);
type_to_value!(u64, BigUnsigned, BigUnsigned);
type_to_value!(f32, Float, Float);
type_to_value!(f64, Double, Double);
type_to_value!(char, Char, Char(None));
type_to_value!(Vec<u8>, Bytes, Blob);
type_to_value!(String, String, Text);

macro_rules! type_ref_to_value {
    ( $type: ty, $name: ident ) => {
        impl From<&$type> for Value {
            fn from(x: &$type) -> Value {
                Value::$name(Some(*x))
            }
        }
    };
}

type_ref_to_value!(bool, Bool);
type_ref_to_value!(i8, TinyInt);
type_ref_to_value!(i16, SmallInt);
type_ref_to_value!(i32, Int);
type_ref_to_value!(i64, BigInt);
type_ref_to_value!(u8, TinyUnsigned);
type_ref_to_value!(u16, SmallUnsigned);
type_ref_to_value!(u32, Unsigned);
type_ref_to_value!(u64, BigUnsigned);
type_ref_to_value!(f32, Float);
type_ref_to_value!(f64, Double);
type_ref_to_value!(char, Char);

/// `From<&T>` for a value type that converts by clone, so a borrowed list
/// (`&ids`, `&uuids`) converts as its items do.
macro_rules! type_ref_clone_to_value {
    ( $($type: ty),+ ) => {$(
        impl From<&$type> for Value {
            fn from(x: &$type) -> Value {
                x.clone().into()
            }
        }
    )+};
}

pub(crate) use type_ref_clone_to_value;

type_ref_clone_to_value!(Vec<u8>);

#[allow(unused_macros)]
macro_rules! type_to_box_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(Box::new(x)))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(*x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::$name
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }

            const COLUMN_KIND: ColumnKind = {
                use ColumnType::*;
                $col_type
            }
            .kind();
        }
    };
}

#[allow(unused_imports)]
pub(crate) use type_to_box_value;
