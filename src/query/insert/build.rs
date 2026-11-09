use super::*;
use crate::{query_statement_builder, query_statement_writer, statements};

query_statement_builder!(InsertStatement, statements::prepare_insert_statement);

impl From<InsertStatement> for QueryStatement {
    fn from(s: InsertStatement) -> Self {
        Self::Insert(s)
    }
}

impl From<InsertStatement> for SubQueryStatement {
    fn from(s: InsertStatement) -> Self {
        Self::InsertStatement(s)
    }
}

query_statement_writer!(InsertStatement);
