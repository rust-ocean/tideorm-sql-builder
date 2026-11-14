//! The types a column can be declared with.

use std::{fmt, sync::Arc};

use crate::types::*;

#[cfg(test)]
#[path = "../tests/unit/column_type_tests.rs"]
mod tests;

/// The type of a column, as an entity declares it.
#[derive(Debug, Clone)]
pub enum ColumnType {
    Char(Option<u32>),
    String(StringLen),
    Text,
    Blob,
    TinyInteger,
    SmallInteger,
    Integer,
    BigInteger,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    Decimal(Option<(u32, u32)>),
    DateTime,
    Timestamp,
    TimestampWithTimeZone,
    Time,
    Date,
    Year,
    Interval(Option<PgInterval>, Option<u32>),
    Binary(u32),
    VarBinary(StringLen),
    Bit(Option<u32>),
    VarBit(u32),
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Uuid,
    Custom(DynIden),
    Enum {
        name: DynIden,
        variants: Vec<DynIden>,
    },
    Array(Arc<ColumnType>),
    Vector(Option<u32>),
    Cidr,
    Inet,
    MacAddr,
}

/// Length for var-char/binary; default to 255 in MySQL if length is not specified
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StringLen {
    /// String size
    N(u32),
    Max,
    #[default]
    None,
}

impl PartialEq for ColumnType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Char(l0), Self::Char(r0))
            | (Self::Bit(l0), Self::Bit(r0))
            | (Self::Vector(l0), Self::Vector(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) | (Self::VarBinary(l0), Self::VarBinary(r0)) => {
                l0 == r0
            }
            (Self::Decimal(l0), Self::Decimal(r0)) | (Self::Money(l0), Self::Money(r0)) => l0 == r0,
            (Self::Interval(l0, l1), Self::Interval(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Binary(l0), Self::Binary(r0)) | (Self::VarBit(l0), Self::VarBit(r0)) => l0 == r0,
            (Self::Custom(l0), Self::Custom(r0)) => l0.to_string() == r0.to_string(),
            (
                Self::Enum {
                    name: l_name,
                    variants: l_variants,
                },
                Self::Enum {
                    name: r_name,
                    variants: r_variants,
                },
            ) => {
                l_name.to_string() == r_name.to_string()
                    && l_variants
                        .iter()
                        .map(|v| v.to_string())
                        .eq(r_variants.iter().map(|v| v.to_string()))
            }
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

/// What a column holds, as far as a `const` can tell: a [`ColumnType`]
/// reduced to the kinds that checks made at compile time read, through
/// [`ValueType::COLUMN_KIND`](crate::ValueType::COLUMN_KIND).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnKind {
    /// An integer of up to 32 bits, signed or not.
    Integer,
    /// A 64-bit integer, signed or not.
    BigInteger,
    /// A floating-point number: `REAL`/`FLOAT` or `DOUBLE`.
    Float,
    /// A UUID.
    Uuid,
    /// Text: `CHAR`, `VARCHAR` or `TEXT`.
    Text,
    /// A JSON document: `JSON` or `JSONB`.
    Json,
    /// Any other type.
    Other,
}

impl ColumnType {
    /// The kind of value the column holds.
    pub const fn kind(&self) -> ColumnKind {
        match self {
            Self::TinyInteger
            | Self::SmallInteger
            | Self::Integer
            | Self::TinyUnsigned
            | Self::SmallUnsigned
            | Self::Unsigned => ColumnKind::Integer,
            Self::BigInteger | Self::BigUnsigned => ColumnKind::BigInteger,
            Self::Float | Self::Double => ColumnKind::Float,
            Self::Uuid => ColumnKind::Uuid,
            Self::Char(_) | Self::String(_) | Self::Text => ColumnKind::Text,
            Self::Json | Self::JsonBinary => ColumnKind::Json,
            _ => ColumnKind::Other,
        }
    }

    pub fn custom<T>(ty: T) -> ColumnType
    where
        T: IntoIden,
    {
        ColumnType::Custom(ty.into_iden())
    }

    pub fn string(length: Option<u32>) -> ColumnType {
        match length {
            Some(s) => ColumnType::String(StringLen::N(s)),
            None => ColumnType::String(StringLen::None),
        }
    }

    pub fn var_binary(length: u32) -> ColumnType {
        ColumnType::VarBinary(StringLen::N(length))
    }
}

// All interval fields
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PgInterval {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    YearToMonth,
    DayToHour,
    DayToMinute,
    DayToSecond,
    HourToMinute,
    HourToSecond,
    MinuteToSecond,
}

// All possible inputs to DATE_TRUNC (https://www.postgresql.org/docs/current/functions-datetime.html#FUNCTIONS-DATETIME-TRUNC)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PgDateTruncUnit {
    Microseconds,
    Milliseconds,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
    Decade,
    Century,
    Millennium,
}

impl fmt::Display for PgDateTruncUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            PgDateTruncUnit::Microseconds => "microseconds",
            PgDateTruncUnit::Milliseconds => "milliseconds",
            PgDateTruncUnit::Second => "second",
            PgDateTruncUnit::Minute => "minute",
            PgDateTruncUnit::Hour => "hour",
            PgDateTruncUnit::Day => "day",
            PgDateTruncUnit::Week => "week",
            PgDateTruncUnit::Month => "month",
            PgDateTruncUnit::Quarter => "quarter",
            PgDateTruncUnit::Year => "year",
            PgDateTruncUnit::Decade => "decade",
            PgDateTruncUnit::Century => "century",
            PgDateTruncUnit::Millennium => "millennium",
        };
        f.write_str(text)
    }
}
