use super::*;
use crate::values;

type_to_box_value!(Json, Json, JsonBinary);
type_ref_clone_to_value!(Json);

impl Value {
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub fn as_ref_json(&self) -> Option<&Json> {
        match self {
            Self::Json(v) => v.as_deref(),
            _ => panic!("not Value::Json"),
        }
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
pub fn value_to_json_value(value: &Value) -> Result<Json, ValueTypeErr> {
    Ok(match value {
        Value::Bool(None)
        | Value::TinyInt(None)
        | Value::SmallInt(None)
        | Value::Int(None)
        | Value::BigInt(None)
        | Value::TinyUnsigned(None)
        | Value::SmallUnsigned(None)
        | Value::Unsigned(None)
        | Value::BigUnsigned(None)
        | Value::Float(None)
        | Value::Double(None)
        | Value::String(None)
        | Value::Char(None)
        | Value::Bytes(None)
        | Value::Json(None)
        | Value::Enum(OptionEnum::None(_))
        | Value::ChronoDate(None)
        | Value::ChronoTime(None)
        | Value::ChronoDateTime(None)
        | Value::ChronoDateTimeWithTimeZone(None)
        | Value::ChronoDateTimeUtc(None)
        | Value::ChronoDateTimeLocal(None)
        | Value::Decimal(None)
        | Value::Uuid(None)
        | Value::Array(_, None) => Json::Null,

        Value::Bool(Some(b)) => Json::Bool(*b),
        Value::TinyInt(Some(v)) => (*v).into(),
        Value::SmallInt(Some(v)) => (*v).into(),
        Value::Int(Some(v)) => (*v).into(),
        Value::BigInt(Some(v)) => (*v).into(),
        Value::TinyUnsigned(Some(v)) => (*v).into(),
        Value::SmallUnsigned(Some(v)) => (*v).into(),
        Value::Unsigned(Some(v)) => (*v).into(),
        Value::BigUnsigned(Some(v)) => (*v).into(),
        Value::Float(Some(v)) => (*v).into(),
        Value::Double(Some(v)) => (*v).into(),
        Value::String(Some(s)) => Json::String(s.clone()),
        Value::Enum(OptionEnum::Some(v)) => Json::String(v.value.to_string()),
        Value::Char(Some(v)) => Json::String(v.to_string()),
        Value::Bytes(Some(s)) => Json::String(
            std::str::from_utf8(s)
                .map_err(|_| ValueTypeErr)?
                .to_string(),
        ),
        Value::Json(Some(v)) => v.as_ref().clone(),

        Value::ChronoDate(Some(_))
        | Value::ChronoTime(Some(_))
        | Value::ChronoDateTime(Some(_))
        | Value::ChronoDateTimeWithTimeZone(Some(_))
        | Value::ChronoDateTimeUtc(Some(_))
        | Value::ChronoDateTimeLocal(Some(_)) => {
            values::value_to_string(&CommonSqlQueryBuilder, value)
                .map_err(|_| ValueTypeErr)?
                .into()
        }

        Value::Decimal(Some(v)) => {
            use rust_decimal::prelude::ToPrimitive;
            v.to_f64().unwrap().into()
        }

        Value::Uuid(Some(v)) => Json::String(v.to_string()),
        Value::Array(_, Some(v)) => Json::Array(
            v.iter()
                .map(value_to_json_value)
                .collect::<Result<_, _>>()?,
        ),
    })
}
