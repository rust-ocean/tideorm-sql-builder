use crate::{
    ConditionHolder, IntoColumnRef, IntoTableRef, OrderExpr, QueryStatement, ReturningClause,
    SubQueryStatement, TableRef, Value, WithClause, WithQuery, conditional_statement,
    ordered_statement, query_statement_builder, query_statement_writer, statements,
};

/// Delete existing rows from the table
///
/// # Examples
///
/// ```
/// use sql_builder::*;
///
/// let query = Query::delete()
///     .from_table("glyph")
///     .cond_where(any![
///         Expr::col("id").lt(1),
///         Expr::col("id").gt(10),
///     ])
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"DELETE FROM `glyph` WHERE `id` < 1 OR `id` > 10"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder).unwrap(),
///     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder).unwrap(),
///     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) r#where: ConditionHolder,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) returning: Option<ReturningClause>,
    pub(crate) with: Option<WithClause>,
}

impl DeleteStatement {
    /// Construct a new [`DeleteStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            r#where: std::mem::take(&mut self.r#where),
            orders: std::mem::take(&mut self.orders),
            limit: self.limit.take(),
            returning: self.returning.take(),
            with: self.with.take(),
        }
    }

    /// Specify which table to delete from.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::delete()
    ///     .from_table("glyph")
    ///     .and_where(Expr::col("id").eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1"#
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn from_table<T>(&mut self, tbl_ref: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// Limit number of deleted rows. PostgreSQL refuses to render a limit or an
    /// order on a `DELETE`.
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit.into());
        self
    }

    /// RETURNING expressions.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::delete()
    ///     .from_table("glyph")
    ///     .and_where(Expr::col("id").eq(1))
    ///     .returning(Query::returning().columns(["id"]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1 RETURNING `id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// ```
    pub fn returning(&mut self, returning_cols: ReturningClause) -> &mut Self {
        self.returning = Some(returning_cols);
        self
    }

    /// RETURNING expressions for a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::delete()
    ///     .from_table("glyph")
    ///     .and_where(Expr::col("id").eq(1))
    ///     .returning_col("id")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1 RETURNING `id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id""#
    /// );
    /// ```
    pub fn returning_col<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoColumnRef,
    {
        self.returning(ReturningClause::Columns(vec![col.into_column_ref()]))
    }

    /// RETURNING expressions all columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::delete()
    ///     .from_table("glyph")
    ///     .and_where(Expr::col("id").eq(1))
    ///     .returning_all()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"DELETE FROM `glyph` WHERE `id` = 1 RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    /// );
    /// ```
    pub fn returning_all(&mut self) -> &mut Self {
        self.returning(ReturningClause::All)
    }

    /// Create a [WithQuery] by specifying a [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{IntoCondition, IntoIden, *};
    ///
    /// let select = SelectStatement::new()
    ///         .columns(["id"])
    ///         .from("glyph")
    ///         .and_where(Expr::col("image").like("0%"))
    ///         .to_owned();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column("id")
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let update = DeleteStatement::new()
    ///         .from_table("glyph")
    ///         .and_where(Expr::col("id").in_subquery(SelectStatement::new().column("id").from("cte").to_owned()))
    ///         .to_owned();
    ///     let query = update.with(with_clause);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH `cte` (`id`) AS (SELECT `id` FROM `glyph` WHERE `image` LIKE '0%') DELETE FROM `glyph` WHERE `id` IN (SELECT `id` FROM `cte`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// ```
    pub fn with(self, clause: WithClause) -> WithQuery {
        clause.query(self)
    }

    /// Create a Common Table Expression by specifying a [CommonTableExpression][crate::CommonTableExpression]
    /// or [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{IntoCondition, IntoIden, *};
    ///
    /// let select = SelectStatement::new()
    ///         .columns(["id"])
    ///         .from("glyph")
    ///         .and_where(Expr::col("image").like("0%"))
    ///         .to_owned();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column("id")
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let query = DeleteStatement::new()
    ///         .with_cte(with_clause)
    ///         .from_table("glyph")
    ///         .and_where(Expr::col("id").in_subquery(SelectStatement::new().column("id").from("cte").to_owned()))
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH `cte` (`id`) AS (SELECT `id` FROM `glyph` WHERE `image` LIKE '0%') DELETE FROM `glyph` WHERE `id` IN (SELECT `id` FROM `cte`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') DELETE FROM "glyph" WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// ```
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(clause.into());
        self
    }
}

query_statement_builder!(DeleteStatement, statements::prepare_delete_statement);

impl From<DeleteStatement> for QueryStatement {
    fn from(s: DeleteStatement) -> Self {
        Self::Delete(s)
    }
}

impl From<DeleteStatement> for SubQueryStatement {
    fn from(s: DeleteStatement) -> Self {
        Self::DeleteStatement(s)
    }
}

query_statement_writer!(DeleteStatement);
ordered_statement!(DeleteStatement, orders);
conditional_statement!(DeleteStatement);
