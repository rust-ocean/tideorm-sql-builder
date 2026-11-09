pub(crate) mod dialect;
pub(crate) mod query;

use super::*;

/// Postgres query builder.
#[derive(Default, Debug)]
pub struct PostgresQueryBuilder;

const QUOTE: Quote = Quote(b'"', b'"');

impl QuotedBuilder for PostgresQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

// https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-BACKSLASH-TABLE
impl EscapeBuilder for PostgresQueryBuilder {
    fn needs_escape(&self, s: &str) -> bool {
        s.chars().any(|c| match c {
            '\x08' | '\x0C' | '\n' | '\r' | '\t' | '\\' | '\'' | '\0' => true,
            c if c.is_ascii_control() => true,
            _ => false,
        })
    }

    fn write_escaped(&self, buffer: &mut impl Write, string: &str) {
        for c in string.chars() {
            match c {
                '\x08' => buffer.write_str(r#"\b"#),
                '\x0C' => buffer.write_str(r#"\f"#),
                '\n' => buffer.write_str(r"\n"),
                '\r' => buffer.write_str(r"\r"),
                '\t' => buffer.write_str(r"\t"),
                '\\' => buffer.write_str(r#"\\"#),
                '\'' => buffer.write_str(r#"\'"#),
                '\0' => buffer.write_str(r#"\0"#),
                c if c.is_ascii_control() => write!(buffer, "\\{:03o}", c as u32),
                _ => buffer.write_char(c),
            }
            .unwrap();
        }
    }
}

impl TableRefBuilder for PostgresQueryBuilder {}

#[cfg(test)]
#[path = "../../../tests/unit/postgres_backend_tests.rs"]
mod tests;
