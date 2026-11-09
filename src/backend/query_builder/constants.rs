use super::*;

pub(super) fn write_string_quoted<B: QueryBuilder>(
    builder: &B,
    string: &str,
    buffer: &mut impl Write,
) {
    buffer.write_str("'").unwrap();
    builder.write_escaped(buffer, string);
    buffer.write_str("'").unwrap();
}

pub(super) fn write_bytes<B: QueryBuilder>(_builder: &B, bytes: &[u8], buffer: &mut impl Write) {
    buffer.write_str("x'").unwrap();
    for b in bytes {
        write!(buffer, "{b:02X}").unwrap()
    }
    buffer.write_str("'").unwrap();
}

pub(super) fn if_null_function<B: QueryBuilder>(_builder: &B) -> &str {
    "IFNULL"
}

pub(super) fn greatest_function<B: QueryBuilder>(_builder: &B) -> &str {
    "GREATEST"
}

pub(super) fn least_function<B: QueryBuilder>(_builder: &B) -> &str {
    "LEAST"
}

pub(super) fn char_length_function<B: QueryBuilder>(_builder: &B) -> &str {
    "CHAR_LENGTH"
}

pub(super) fn random_function<B: QueryBuilder>(_builder: &B) -> &str {
    // Returning it with parens as part of the name because the tuple preparer can't deal with empty lists
    "RANDOM"
}

pub(super) fn lock_phrase<B: QueryBuilder>(_builder: &B, lock_type: LockType) -> &'static str {
    match lock_type {
        LockType::Update => "FOR UPDATE",
        LockType::NoKeyUpdate => "FOR NO KEY UPDATE",
        LockType::Share => "FOR SHARE",
        LockType::KeyShare => "FOR KEY SHARE",
    }
}

pub(super) fn insert_default_keyword<B: QueryBuilder>(_builder: &B) -> &str {
    "(DEFAULT)"
}

pub(super) fn insert_default_values<B: QueryBuilder>(
    builder: &B,
    num_rows: usize,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str("VALUES ").unwrap();
    if num_rows > 0 {
        sql.write_str(builder.insert_default_keyword()).unwrap();

        for _ in 1..num_rows {
            sql.write_str(", ").unwrap();
            sql.write_str(builder.insert_default_keyword()).unwrap();
        }
    }
    Ok(())
}
