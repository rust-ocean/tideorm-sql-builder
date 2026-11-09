use super::*;

pub(super) fn prepare_constant<B: QueryBuilder>(
    builder: &B,
    value: &Value,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    builder.write_value(sql, value)
}

pub(super) fn prepare_values_list<B: QueryBuilder>(
    builder: &B,
    value_tuples: &[ValueTuple],
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str("VALUES ").unwrap();
    let mut tuples = value_tuples.iter();
    join_io!(
        tuples,
        value_tuple,
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            sql.write_str(builder.values_list_tuple_prefix()).unwrap();
            sql.write_str("(").unwrap();

            let mut values = value_tuple.clone().into_iter();
            join_io!(
                values,
                value,
                join {
                    sql.write_str(", ").unwrap();
                },
                do {
                    builder.prepare_value(value, sql)?;
                }
            );

            sql.write_str(")").unwrap();
        }
    );
    Ok(())
}

pub(super) fn prepare_values_rows<B: QueryBuilder>(
    builder: &B,
    values: &[Values],
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str("VALUES ").unwrap();
    let mut rows = values.iter();
    join_io!(
        rows,
        row,
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            sql.write_str("(").unwrap();

            let mut vals = row.clone().into_iter();
            join_io!(
                vals,
                val,
                join {
                    sql.write_str(", ").unwrap();
                },
                do {
                    builder.prepare_value(val, sql)?;
                }
            );

            sql.write_str(")").unwrap();
        }
    );
    Ok(())
}

pub(super) fn prepare_tuple<B: QueryBuilder>(
    builder: &B,
    exprs: &[Expr],
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str("(").unwrap();
    for (i, expr) in exprs.iter().enumerate() {
        if i != 0 {
            sql.write_str(", ").unwrap();
        }
        builder.prepare_expr(expr, sql)?;
    }
    sql.write_str(")").unwrap();
    Ok(())
}

pub(super) fn prepare_keyword<B: QueryBuilder>(
    _builder: &B,
    keyword: &Keyword,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match keyword {
        Keyword::Null => sql.write_str("NULL").unwrap(),
        Keyword::CurrentDate => sql.write_str("CURRENT_DATE").unwrap(),
        Keyword::CurrentTime => sql.write_str("CURRENT_TIME").unwrap(),
        Keyword::CurrentTimestamp => sql.write_str("CURRENT_TIMESTAMP").unwrap(),
        Keyword::Default => sql.write_str("DEFAULT").unwrap(),
        Keyword::Custom(iden) => sql.write_str(&iden.0).unwrap(),
    }
    Ok(())
}

pub(crate) fn value_to_string<B: QueryBuilder>(
    builder: &B,
    v: &Value,
) -> Result<String, RenderError> {
    value_to_string_common(builder, v)
}

pub(super) fn value_to_string_common<B: QueryBuilder>(
    builder: &B,
    v: &Value,
) -> Result<String, RenderError> {
    let mut s = String::new();
    builder.write_value(&mut s, v)?;
    Ok(s)
}

pub(super) fn write_value<B: QueryBuilder>(
    builder: &B,
    buf: &mut impl SqlWriter,
    value: &Value,
) -> Result<(), RenderError> {
    write_value_common(builder, buf, value)
}

pub(crate) fn write_value_common<B: QueryBuilder>(
    builder: &B,
    buf: &mut impl SqlWriter,
    value: &Value,
) -> Result<(), RenderError> {
    match value {
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
        | Value::Enum(OptionEnum::None(_))
        | Value::Json(None)
        | Value::ChronoDate(None)
        | Value::ChronoTime(None)
        | Value::ChronoDateTime(None)
        | Value::ChronoDateTimeUtc(None)
        | Value::ChronoDateTimeLocal(None)
        | Value::ChronoDateTimeWithTimeZone(None)
        | Value::Decimal(None)
        | Value::Uuid(None)
        | Value::Array(_, None) => buf.write_str("NULL").unwrap(),

        Value::Bool(Some(b)) => buf.write_str(if *b { "TRUE" } else { "FALSE" }).unwrap(),
        Value::TinyInt(Some(v)) => {
            write_int(buf, *v);
        }
        Value::SmallInt(Some(v)) => {
            write_int(buf, *v);
        }
        Value::Int(Some(v)) => {
            write_int(buf, *v);
        }
        Value::BigInt(Some(v)) => {
            write_int(buf, *v);
        }
        Value::TinyUnsigned(Some(v)) => {
            write_int(buf, *v);
        }
        Value::SmallUnsigned(Some(v)) => {
            write_int(buf, *v);
        }
        Value::Unsigned(Some(v)) => {
            write_int(buf, *v);
        }
        Value::BigUnsigned(Some(v)) => {
            write_int(buf, *v);
        }
        Value::Float(Some(v)) => write!(buf, "{v}").unwrap(),
        Value::Double(Some(v)) => write!(buf, "{v}").unwrap(),
        Value::String(Some(v)) => builder.write_string_quoted(v, buf),
        Value::Enum(OptionEnum::Some(v)) => builder.write_string_quoted(v.value.as_ref(), buf),
        Value::Char(Some(v)) => builder.write_string_quoted(v.encode_utf8(&mut [0; 4]), buf),
        Value::Bytes(Some(v)) => builder.write_bytes(v, buf),

        Value::Json(Some(v)) => builder.write_string_quoted(&v.to_string(), buf),

        Value::ChronoDate(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{}", v.format("%Y-%m-%d")).unwrap();
            buf.write_str("'").unwrap();
        }

        Value::ChronoTime(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{}", v.format("%H:%M:%S%.6f")).unwrap();
            buf.write_str("'").unwrap();
        }

        Value::ChronoDateTime(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{}", v.format("%Y-%m-%d %H:%M:%S%.6f")).unwrap();
            buf.write_str("'").unwrap();
        }

        Value::ChronoDateTimeUtc(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{}", v.format("%Y-%m-%d %H:%M:%S%.6f %:z")).unwrap();
            buf.write_str("'").unwrap();
        }

        Value::ChronoDateTimeLocal(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{}", v.format("%Y-%m-%d %H:%M:%S%.6f %:z")).unwrap();
            buf.write_str("'").unwrap();
        }

        Value::ChronoDateTimeWithTimeZone(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{}", v.format("%Y-%m-%d %H:%M:%S%.6f %:z")).unwrap();
            buf.write_str("'").unwrap();
        }

        Value::Decimal(Some(v)) => write!(buf, "{v}").unwrap(),

        Value::Uuid(Some(v)) => {
            buf.write_str("'").unwrap();
            write!(buf, "{v}").unwrap();
            buf.write_str("'").unwrap();
        }
        Value::Array(_, Some(v)) => {
            if v.is_empty() {
                buf.write_str("'{}'").unwrap();
            } else {
                buf.write_str("ARRAY [").unwrap();

                let mut viter = v.iter();

                if let Some(element) = viter.next() {
                    builder.write_value(buf, element)?;
                }

                for element in viter {
                    buf.write_str(",").unwrap();
                    builder.write_value(buf, element)?;
                }
                buf.write_str("]").unwrap();
            }
        }
    };

    Ok(())
}
