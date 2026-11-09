pub(crate) mod dialect;
mod json;
pub(crate) mod query;

use super::*;

pub use dialect::MariaDbDialect;

/// Mysql query builder.
#[derive(Default, Debug)]
pub struct MysqlQueryBuilder;

const QUOTE: Quote = Quote(b'`', b'`');

pub type MySqlQueryBuilder = MysqlQueryBuilder;

impl QuotedBuilder for MysqlQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for MysqlQueryBuilder {}

impl TableRefBuilder for MysqlQueryBuilder {}

impl PrecedenceDecider for MysqlQueryBuilder {}

impl OperLeftAssocDecider for MysqlQueryBuilder {}
