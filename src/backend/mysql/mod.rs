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

/// The builder traits MySQL and MariaDB share.
macro_rules! mysql_family_builder {
    ($builder:ty) => {
        impl QuotedBuilder for $builder {
            fn quote(&self) -> Quote {
                QUOTE
            }
        }

        impl EscapeBuilder for $builder {}

        impl TableRefBuilder for $builder {}

        impl PrecedenceDecider for $builder {}

        impl OperLeftAssocDecider for $builder {}
    };
}

mysql_family_builder!(MysqlQueryBuilder);
mysql_family_builder!(MariaDbDialect);
