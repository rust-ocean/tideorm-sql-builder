use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};

/// [Postgresql reference](https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-DATETIME-INPUT-TIMES)
///
/// [Mysql reference](https://dev.mysql.com/doc/refman/8.4/en/fractional-seconds.html)
///
/// [Sqlite reference](https://sqlite.org/lang_datefunc.html)
#[test]

fn format_time_constant() {
    use crate::{MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};

    let time = NaiveTime::from_hms_micro_opt(1, 2, 3, 123456)
        .unwrap()
        .into();

    let mut string = String::new();
    macro_rules! compare {
        ($a:ident, $b:literal) => {
            super::values::prepare_constant(&PostgresQueryBuilder, &$a, &mut string).unwrap();
            assert_eq!(string, $b);

            string.clear();

            super::values::prepare_constant(&MysqlQueryBuilder, &$a, &mut string).unwrap();
            assert_eq!(string, $b);

            string.clear();

            super::values::prepare_constant(&SqliteQueryBuilder, &$a, &mut string).unwrap();
            assert_eq!(string, $b);

            string.clear();
        };
    }

    compare!(time, "'01:02:03.123456'");

    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_micro_opt(12, 34, 56, 123456).unwrap();

    let dt = NaiveDateTime::new(d, t);

    let date_time = dt.into();

    compare!(date_time, "'2015-06-03 12:34:56.123456'");

    let date_time_utc = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc).into();

    compare!(date_time_utc, "'2015-06-03 12:34:56.123456 +00:00'");

    let date_time_tz = DateTime::<FixedOffset>::from_naive_utc_and_offset(
        dt,
        FixedOffset::east_opt(8 * 3600).unwrap(),
    )
    .into();

    compare!(date_time_tz, "'2015-06-03 20:34:56.123456 +08:00'");
}

#[test]
fn prepare_type_ref_escape_array() {
    use crate::{PostgresQueryBuilder, TypeRef};

    let mut buf = String::new();
    let test_cases = [
        ("text", r#""text""#),
        ("text[]", r#""text"[]"#),
        ("text[][]", r#""text"[][]"#),
        ("text[][][]", r#""text"[][][]"#),
        ("text[][][][]", r#""text"[][][][]"#),
        ("text[][][][][]", r#""text"[][][][][]"#),
    ];
    for (ty, expect) in test_cases {
        super::functions::prepare_type_ref(&PostgresQueryBuilder, &TypeRef::from(ty), &mut buf);
        assert_eq!(buf, expect);
        buf.clear();
    }
}

#[test]
fn prepare_array_null_and_empty() {
    use crate::{ArrayType, PostgresQueryBuilder, QueryBuilder, Value};

    let mut string = String::new();
    PostgresQueryBuilder
        .prepare_value(Value::Array(ArrayType::String, None), &mut string)
        .unwrap();
    assert_eq!(string, "NULL");

    string.clear();
    PostgresQueryBuilder
        .prepare_value(Vec::<String>::new().into(), &mut string)
        .unwrap();
    assert_eq!(string, "'{}'");
}
