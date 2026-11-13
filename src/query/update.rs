mod getters;

use crate::{
    ConditionHolder, DynIden, Expr, IntoColumnRef, IntoIden, IntoTableRef, OrderExpr,
    QueryStatement, ReturningClause, SubQueryStatement, TableRef, Value, WithClause, WithQuery,
    conditional_statement, ordered_statement, query_statement_builder, query_statement_writer,
    statements,
};

/// Update existing rows in the table
///
/// # Examples
///
/// ```
/// use sql_builder::*;
///
/// let query = Query::update()
///     .table("glyph")
///     .values([("aspect", 1.23.into()), ("image", "123".into())])
///     .and_where(Expr::col("id").eq(1))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"UPDATE `glyph` SET `aspect` = 1.23, `image` = '123' WHERE `id` = 1"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder).unwrap(),
///     r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder).unwrap(),
///     r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateStatement {
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) from: Vec<TableRef>,
    pub(crate) values: Vec<(DynIden, Box<Expr>)>,
    pub(crate) r#where: ConditionHolder,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) returning: Option<ReturningClause>,
    pub(crate) with: Option<WithClause>,
}

impl UpdateStatement {
    /// Construct a new [`UpdateStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take(&mut self) -> Self {
        Self {
            table: self.table.take(),
            from: std::mem::take(&mut self.from),
            values: std::mem::take(&mut self.values),
            r#where: std::mem::take(&mut self.r#where),
            orders: std::mem::take(&mut self.orders),
            limit: self.limit.take(),
            returning: self.returning.take(),
            with: self.with.take(),
        }
    }

    /// Specify which table to update; see [`UpdateStatement::values`] for an
    /// example.
    #[allow(clippy::wrong_self_convention)]
    pub fn table<T>(&mut self, tbl_ref: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// Update using data from another table (`UPDATE .. FROM ..`).
    ///
    /// # MySQL Notes
    ///
    /// MySQL has no `UPDATE .. FROM`: it gets `UPDATE .. JOIN .. ON <where>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .value("tokens", Expr::column(("character", "character")))
    ///     .from("character")
    ///     .cond_where(
    ///         Expr::col(("glyph", "image"))
    ///             .eq(Expr::col(("character", "user_data"))),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     "UPDATE `glyph` JOIN `character` ON `glyph`.`image` = `character`.`user_data` SET `glyph`.`tokens` = `character`.`character`"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "tokens" = "character"."character" FROM "character" WHERE "glyph"."image" = "character"."user_data""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "tokens" = "character"."character" FROM "character" WHERE "glyph"."image" = "character"."user_data""#
    /// );
    /// ```
    pub fn from<R>(&mut self, tbl_ref: R) -> &mut Self
    where
        R: IntoTableRef,
    {
        self.from_from(tbl_ref.into_table_ref())
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_from(&mut self, select: TableRef) -> &mut Self {
        self.from.push(select);
        self
    }

    /// Update column values. To set multiple column-value pairs at once.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .values([
    ///         ("aspect", 2.1345.into()),
    ///         ("image", "235m".into()),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m'"#
    /// );
    /// ```
    pub fn values<T, I>(&mut self, values: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = (T, Expr)>,
    {
        for (k, v) in values.into_iter() {
            self.values.push((k.into_iden(), Box::new(v)));
        }
        self
    }

    /// Update column value by [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .value("aspect", Expr::cust("60 * 24 * 24"))
    ///     .values([
    ///         ("image", "24B0E11951B03B07F8300FD003983F03F0780060".into()),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"UPDATE `glyph` SET `aspect` = 60 * 24 * 24, `image` = '24B0E11951B03B07F8300FD003983F03F0780060'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060'"#
    /// );
    ///
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .value("aspect", Expr::value(Value::Int(None)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"UPDATE `glyph` SET `aspect` = NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = NULL"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = NULL"#
    /// );
    /// ```
    pub fn value<C, T>(&mut self, col: C, value: T) -> &mut Self
    where
        C: IntoIden,
        T: Into<Expr>,
    {
        self.values.push((col.into_iden(), Box::new(value.into())));
        self
    }

    /// Limit number of updated rows. PostgreSQL refuses to render a limit or an
    /// order on an `UPDATE`.
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
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .value("aspect", 2.1345)
    ///     .value("image", "235m")
    ///     .returning(Query::returning().columns(["id"]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MariaDbDialect).unwrap(),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' RETURNING `id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' RETURNING "id""#
    /// );
    /// ```
    pub fn returning(&mut self, returning: ReturningClause) -> &mut Self {
        self.returning = Some(returning);
        self
    }

    /// RETURNING expressions for a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .value("aspect", 2.1345)
    ///     .value("image", "235m")
    ///     .returning_col("id")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MariaDbDialect).unwrap(),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' RETURNING `id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' RETURNING "id""#
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
    /// let query = Query::update()
    ///     .table("glyph")
    ///     .value("aspect", 2.1345)
    ///     .value("image", "235m")
    ///     .returning_all()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MariaDbDialect).unwrap(),
    ///     r#"UPDATE `glyph` SET `aspect` = 2.1345, `image` = '235m' RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '235m' RETURNING *"#
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
    ///     let update = UpdateStatement::new()
    ///         .table("glyph")
    ///         .and_where(Expr::col("id").in_subquery(SelectStatement::new().column("id").from("cte").to_owned()))
    ///         .value("aspect", Expr::cust("60 * 24 * 24"))
    ///         .to_owned();
    ///     let query = update.with(with_clause);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH `cte` (`id`) AS (SELECT `id` FROM `glyph` WHERE `image` LIKE '0%') UPDATE `glyph` SET `aspect` = 60 * 24 * 24 WHERE `id` IN (SELECT `id` FROM `cte`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') UPDATE "glyph" SET "aspect" = 60 * 24 * 24 WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') UPDATE "glyph" SET "aspect" = 60 * 24 * 24 WHERE "id" IN (SELECT "id" FROM "cte")"#
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
    ///     let query = UpdateStatement::new()
    ///         .table("glyph")
    ///         .and_where(Expr::col("id").in_subquery(SelectStatement::new().column("id").from("cte").to_owned()))
    ///         .value("aspect", Expr::cust("60 * 24 * 24"))
    ///         .with_cte(with_clause)
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH `cte` (`id`) AS (SELECT `id` FROM `glyph` WHERE `image` LIKE '0%') UPDATE `glyph` SET `aspect` = 60 * 24 * 24 WHERE `id` IN (SELECT `id` FROM `cte`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') UPDATE "glyph" SET "aspect" = 60 * 24 * 24 WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id") AS (SELECT "id" FROM "glyph" WHERE "image" LIKE '0%') UPDATE "glyph" SET "aspect" = 60 * 24 * 24 WHERE "id" IN (SELECT "id" FROM "cte")"#
    /// );
    /// ```
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(clause.into());
        self
    }
}

query_statement_builder!(UpdateStatement, statements::prepare_update_statement);

impl From<UpdateStatement> for QueryStatement {
    fn from(s: UpdateStatement) -> Self {
        Self::Update(s)
    }
}

impl From<UpdateStatement> for SubQueryStatement {
    fn from(s: UpdateStatement) -> Self {
        Self::UpdateStatement(s)
    }
}

query_statement_writer!(UpdateStatement);
ordered_statement!(UpdateStatement, orders);
conditional_statement!(UpdateStatement);
