use super::*;
use crate::{
    conditional_statement, ordered_statement, query_statement_builder, query_statement_writer,
    statements,
};

query_statement_builder!(SelectStatement, statements::prepare_select_statement);

impl From<SelectStatement> for QueryStatement {
    fn from(s: SelectStatement) -> Self {
        Self::Select(s)
    }
}

impl From<SelectStatement> for SubQueryStatement {
    fn from(s: SelectStatement) -> Self {
        Self::SelectStatement(s)
    }
}

query_statement_writer!(SelectStatement);
ordered_statement!(SelectStatement, orders);
conditional_statement!(SelectStatement);
