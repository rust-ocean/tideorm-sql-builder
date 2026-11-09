use std::{borrow::Cow, ops::Deref};

use crate::*;

const QUOTE: Quote = Quote(b'"', b'"');

mod common;
pub(crate) mod conflict;
mod constants;
pub(crate) mod expressions;
pub(crate) mod functions;
pub(crate) mod statements;
pub(crate) mod values;
pub(crate) use common::*;

pub trait QueryBuilder:
    QuotedBuilder + EscapeBuilder + TableRefBuilder + OperLeftAssocDecider + PrecedenceDecider + Sized
{
    /// The type of placeholder the builder uses for values, and whether it is numbered.
    fn placeholder(&self) -> (&'static str, bool) {
        statements::placeholder(self)
    }

    /// Whose lexical rules SQL text written for this builder follows: where
    /// its strings, comments and markers begin and end.
    fn lexer_dialect(&self) -> crate::lexer::Dialect;

    /// Prefix for tuples in VALUES list (e.g. ROW for MySQL)
    fn values_list_tuple_prefix(&self) -> &str {
        statements::values_list_tuple_prefix(self)
    }

    fn prepare_union_statement(
        &self,
        union_type: UnionType,
        select_statement: &SelectStatement,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        statements::prepare_union_statement(self, union_type, select_statement, sql)
    }

    // Translate the LIMIT and OFFSET expression in [`SelectStatement`]
    fn prepare_select_limit_offset(
        &self,
        select: &SelectStatement,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        prepare_limit_offset(self, select, None, sql)
    }

    #[allow(unused_variables)]
    fn prepare_update_join(
        &self,
        from: &[TableRef],
        condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        // MySQL specific
        Ok(())
    }

    fn prepare_update_from(
        &self,
        from: &[TableRef],
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        statements::prepare_update_from(self, from, sql)
    }

    fn prepare_update_column(
        &self,
        table: &Option<Box<TableRef>>,
        from: &[TableRef],
        column: &DynIden,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        statements::prepare_update_column(self, table, from, column, sql)
    }

    fn prepare_update_condition(
        &self,
        from: &[TableRef],
        condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        statements::prepare_update_condition(self, from, condition, sql)
    }

    /// Translate [`Expr`] into SQL statement.
    fn prepare_expr(
        &self,
        simple_expr: &Expr,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        expressions::prepare_expr(self, simple_expr, sql)
    }

    /// Translate [`SelectDistinct`] into SQL statement.
    fn prepare_select_distinct(
        &self,
        select_distinct: &SelectDistinct,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        expressions::prepare_select_distinct(self, select_distinct, sql)
    }

    /// Translate [`LockType`] into SQL statement.
    fn prepare_select_lock(
        &self,
        lock: &LockClause,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        expressions::prepare_select_lock(self, lock, sql)
    }

    /// Translate [`BinOper`] into SQL statement.
    fn prepare_bin_oper(
        &self,
        bin_oper: &BinOper,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        expressions::prepare_bin_oper(self, bin_oper, sql)
    }

    /// Translate [`SubQueryOper`] into SQL statement.
    fn prepare_sub_query_oper(
        &self,
        oper: &SubQueryOper,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        expressions::prepare_sub_query_oper(self, oper, sql);
        Ok(())
    }

    /// Translate [`QueryStatement`] into SQL statement.
    fn prepare_query_statement(
        &self,
        query: &SubQueryStatement,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        query.prepare_statement(self, sql)
    }

    fn prepare_select_into(
        &self,
        into_table: &SelectInto,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError>;

    fn prepare_with_query_clause_materialization(
        &self,
        cte: &CommonTableExpression,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        functions::prepare_with_query_clause_materialization(self, cte, sql)
    }

    fn prepare_function_name(
        &self,
        function: &Func,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        functions::prepare_function_name_common(self, function, sql)
    }

    /// Translate [`JoinType`] into SQL statement.
    fn prepare_join_type(
        &self,
        join_type: &JoinType,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        functions::prepare_join_type_common(self, join_type, sql)
    }

    /// Translate [`OrderExpr`] into SQL statement.
    fn prepare_order_expr(
        &self,
        order_expr: &OrderExpr,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        functions::prepare_order_expr(self, order_expr, sql)
    }

    /// Write [`Value`] into SQL statement as parameter.
    fn prepare_value(&self, value: Value, sql: &mut impl SqlWriter) -> Result<(), RenderError> {
        sql.push_param(value, self)
    }

    #[doc(hidden)]
    fn write_value(&self, buf: &mut impl SqlWriter, value: &Value) -> Result<(), RenderError> {
        values::write_value(self, buf, value)
    }

    #[doc(hidden)]
    /// Write ON CONFLICT target
    fn prepare_on_conflict_target(
        &self,
        on_conflict_targets: &OnConflictTarget,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        conflict::prepare_on_conflict_target(self, on_conflict_targets, sql)
    }

    #[doc(hidden)]
    /// Write ON CONFLICT action
    fn prepare_on_conflict_action(
        &self,
        on_conflict_action: &Option<OnConflictAction>,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        conflict::prepare_on_conflict_action(self, on_conflict_action, sql)
    }

    #[doc(hidden)]
    /// Write ON CONFLICT keywords
    fn prepare_on_conflict_keywords(&self, sql: &mut impl SqlWriter) -> Result<(), RenderError> {
        conflict::prepare_on_conflict_keywords(self, sql);
        Ok(())
    }

    #[doc(hidden)]
    /// Write ON CONFLICT keywords
    fn prepare_on_conflict_do_update_keywords(
        &self,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        conflict::prepare_on_conflict_do_update_keywords(self, sql);
        Ok(())
    }

    #[doc(hidden)]
    /// Write ON CONFLICT update action by retrieving value from the excluded table
    fn prepare_on_conflict_excluded_table(
        &self,
        col: &DynIden,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        conflict::prepare_on_conflict_excluded_table(self, col, sql)
    }

    #[doc(hidden)]
    /// Write ON CONFLICT conditions
    fn prepare_on_conflict_condition(
        &self,
        on_conflict_condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        conflict::prepare_on_conflict_condition(self, on_conflict_condition, sql)
    }

    #[doc(hidden)]
    /// Write the WHERE of an ON CONFLICT action
    fn prepare_on_conflict_action_condition(
        &self,
        condition: &ConditionHolder,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        self.prepare_on_conflict_condition(condition, sql)
    }

    #[doc(hidden)]
    /// Write `INSERT`, or `REPLACE` when `replace` is set
    fn prepare_insert(&self, replace: bool, sql: &mut impl SqlWriter) -> Result<(), RenderError> {
        functions::prepare_insert(self, replace, sql);
        Ok(())
    }

    #[doc(hidden)]
    /// Write the ORDER BY and LIMIT of an UPDATE or DELETE
    fn prepare_mutation_order_limit(
        &self,
        orders: &[OrderExpr],
        limit: &Option<Value>,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        statements::prepare_mutation_order_limit(self, orders, limit, sql)
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut impl Write) {
        constants::write_string_quoted(self, string, buffer);
    }

    #[doc(hidden)]
    /// Write bytes enclosed with engine specific byte syntax
    fn write_bytes(&self, bytes: &[u8], buffer: &mut impl Write) {
        constants::write_bytes(self, bytes, buffer);
    }

    #[doc(hidden)]
    /// The name of the function that represents the "if null" condition.
    fn if_null_function(&self) -> &str {
        constants::if_null_function(self)
    }

    #[doc(hidden)]
    /// The name of the function that represents the "greatest" function.
    fn greatest_function(&self) -> &str {
        constants::greatest_function(self)
    }

    #[doc(hidden)]
    /// The name of the function that represents the "least" function.
    fn least_function(&self) -> &str {
        constants::least_function(self)
    }

    #[doc(hidden)]
    /// The name of the function that returns the char length.
    fn char_length_function(&self) -> &str {
        constants::char_length_function(self)
    }

    #[doc(hidden)]
    /// The name of the function that returns a random number
    fn random_function(&self) -> &str {
        constants::random_function(self)
    }

    #[doc(hidden)]
    /// The name of the function that returns the lock phrase including the leading 'FOR'
    fn lock_phrase(&self, lock_type: LockType) -> &'static str {
        constants::lock_phrase(self, lock_type)
    }

    /// The keywords for insert default row.
    fn insert_default_keyword(&self) -> &str {
        constants::insert_default_keyword(self)
    }

    /// Write insert default rows expression.
    fn insert_default_values(
        &self,
        num_rows: usize,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        constants::insert_default_values(self, num_rows, sql)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/query_builder_tests.rs"]
mod tests;
