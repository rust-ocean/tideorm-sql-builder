pub(crate) mod dialect;
mod json;
pub(crate) mod query;

use super::*;

/// Sqlite query builder.
#[derive(Default, Debug)]
pub struct SqliteQueryBuilder;

const QUOTE: Quote = Quote(b'"', b'"');

impl QuotedBuilder for SqliteQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for SqliteQueryBuilder {
    fn write_escaped(&self, buffer: &mut impl Write, string: &str) {
        for char in string.chars() {
            match char {
                '\'' => buffer.write_str("''"),
                c => buffer.write_char(c),
            }
            .unwrap()
        }
    }
}

impl TableRefBuilder for SqliteQueryBuilder {}

impl PrecedenceDecider for SqliteQueryBuilder {}

impl OperLeftAssocDecider for SqliteQueryBuilder {}
