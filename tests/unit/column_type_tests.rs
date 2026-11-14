use super::*;

#[test]
fn vectors_compare_their_dimensions() {
    assert_ne!(ColumnType::Vector(Some(3)), ColumnType::Vector(Some(1536)));
    assert_ne!(ColumnType::Vector(Some(3)), ColumnType::Vector(None));
    assert_eq!(ColumnType::Vector(Some(3)), ColumnType::Vector(Some(3)));
    assert_eq!(ColumnType::Vector(None), ColumnType::Vector(None));
}

/// Each value type's `COLUMN_KIND` is the kind of its `column_type()`.
#[test]
fn value_types_declare_the_kind_of_their_column_type() {
    use std::borrow::Cow;

    use crate::ValueType;

    fn agrees<T: ValueType>() {
        assert_eq!(
            T::COLUMN_KIND,
            T::column_type().kind(),
            "{}",
            std::any::type_name::<T>()
        );
    }
    agrees::<bool>();
    agrees::<i8>();
    agrees::<i16>();
    agrees::<i32>();
    agrees::<i64>();
    agrees::<u8>();
    agrees::<u16>();
    agrees::<u32>();
    agrees::<u64>();
    agrees::<f32>();
    agrees::<f64>();
    agrees::<char>();
    agrees::<Vec<u8>>();
    agrees::<String>();
    agrees::<Cow<'_, str>>();
    agrees::<crate::Enum>();
    agrees::<Option<f64>>();
    agrees::<Option<String>>();
    agrees::<serde_json::Value>();
    agrees::<rust_decimal::Decimal>();
    agrees::<uuid::Uuid>();
    agrees::<uuid::fmt::Hyphenated>();
    agrees::<chrono::NaiveDate>();
    agrees::<chrono::NaiveDateTime>();
    agrees::<chrono::DateTime<chrono::Utc>>();
    agrees::<Vec<i32>>();
    assert_eq!(i64::COLUMN_KIND, ColumnKind::BigInteger);
    assert_eq!(<Option<serde_json::Value>>::COLUMN_KIND, ColumnKind::Json);
}
