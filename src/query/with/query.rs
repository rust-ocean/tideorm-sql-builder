use super::*;
use crate::{RenderError, functions, query_statement_writer};

/// A WITH clause can contain one or multiple common table expressions ([CommonTableExpression]).
///
/// You can use this to generate [WithQuery] by calling [WithClause::query].
///
/// These named queries can act as a "query local table" that are materialized during execution and
/// then can be used by the query prefixed with the WITH clause.
///
/// A WITH clause can contain multiple of these [CommonTableExpression]. (Except in the case of
/// recursive WITH query which can only contain one [CommonTableExpression]).
///
/// A [CommonTableExpression] is a name, column names and a query returning data for those columns.
///
/// Some databases (like sqlite) restrict the acceptable kinds of queries inside of the WITH clause
/// common table expressions. These databases only allow [SelectStatement]s to form a common table
/// expression.
///
/// Other databases like postgres allow modification queries (UPDATE, DELETE) inside of the WITH
/// clause but they have to return a table. (They must have a RETURNING clause).
///
/// tideorm-sql-builder doesn't check this or restrict the kind of [CommonTableExpression] that you can create
/// in rust. This means that you can put an UPDATE or DELETE queries into WITH clause and tideorm-sql-builder
/// will succeed in generating that kind of sql query but the execution inside the database will
/// fail because they are invalid.
///
/// It is your responsibility to ensure that the kind of WITH clause that you put together makes
/// sense and valid for that database that you are using.
///
/// NOTE that for recursive WITH queries (in sql: "WITH RECURSIVE") you can only have a
/// single [CommonTableExpression] inside of the WITH clause. That query must match certain
/// requirements:
///   * It is a query of UNION or UNION ALL of two queries.
///   * The first part of the query (the left side of the UNION) must be executable first in itself.
///     It must be non-recursive. (Cannot contain self reference)
///   * The self reference must appear in the right hand side of the UNION.
///   * The query can only have a single self-reference.
///   * Recursive data-modifying statements are not supported, but you can use the results of a
///     recursive SELECT query in a data-modifying statement. (like so: WITH RECURSIVE
///     cte_name(a,b,c,d) AS (SELECT ... UNION SELECT ... FROM ... JOIN cte_name ON ... WHERE ...)
///     DELETE FROM table WHERE table.a = cte_name.a)
///
/// It is mandatory to set the [Self::cte]. With queries must have at least one CTE.
/// Recursive with query generation will panic if you specify more than one CTE.
///
/// # Examples
///
/// ```
/// use sql_builder::{*, IntoCondition, IntoIden};
///
/// let base_query = SelectStatement::new()
///                     .column("id")
///                     .expr(1i32)
///                     .column("next")
///                     .column("value")
///                     .from("table")
///                     .to_owned();
///
/// let cte_referencing = SelectStatement::new()
///                             .column("id")
///                             .expr(Expr::col("depth").add(1i32))
///                             .column("next")
///                             .column("value")
///                             .from("table")
///                             .join(
///                                 JoinType::InnerJoin,
///                                 "cte_traversal",
///                                 Expr::col(("cte_traversal", "next")).equals(("table", "id"))
///                             )
///                             .to_owned();
///
/// let common_table_expression = CommonTableExpression::new()
///             .query(
///                 base_query.clone().union(UnionType::All, cte_referencing).to_owned()
///             )
///             .column("id")
///             .column("depth")
///             .column("next")
///             .column("value")
///             .table_name("cte_traversal")
///             .to_owned();
///
/// let select = SelectStatement::new()
///         .column(Asterisk)
///         .from("cte_traversal")
///         .to_owned();
///
/// let with_clause = WithClause::new()
///         .recursive(true)
///         .cte(common_table_expression)
///         .to_owned();
///
/// let query = select.with(with_clause).to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, 1, `next`, `value` FROM `table` UNION ALL (SELECT `id`, `depth` + 1, `next`, `value` FROM `table` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `table`.`id`)) SELECT * FROM `cte_traversal`"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder).unwrap(),
///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL (SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id")) SELECT * FROM "cte_traversal""#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder).unwrap(),
///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "table" UNION ALL SELECT "id", "depth" + 1, "next", "value" FROM "table" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "table"."id") SELECT * FROM "cte_traversal""#
/// );
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WithClause {
    pub(crate) recursive: bool,
    pub(crate) cte_expressions: Vec<CommonTableExpression>,
}

impl WithClause {
    /// Constructs a new [WithClause].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether this clause is a recursive with clause of not.
    /// If set to true it will generate a 'WITH RECURSIVE' query.
    ///
    /// You can only specify a single [CommonTableExpression] containing a union query
    /// if this is set to true.
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive = recursive;
        self
    }

    /// Add a [CommonTableExpression] to this with clause.
    pub fn cte(&mut self, cte: CommonTableExpression) -> &mut Self {
        self.cte_expressions.push(cte);
        self
    }

    /// You can turn this into a [WithQuery] using this function. The resulting WITH query will
    /// execute the argument query with this WITH clause.
    pub fn query<T>(self, query: T) -> WithQuery
    where
        T: Into<SubQueryStatement>,
    {
        WithQuery::new().with_clause(self).query(query).to_owned()
    }
}

impl From<CommonTableExpression> for WithClause {
    fn from(cte: CommonTableExpression) -> WithClause {
        WithClause::new().cte(cte).to_owned()
    }
}

/// A WITH query. A simple SQL query that has a WITH clause ([WithClause]).
///
/// The [WithClause] can contain one or multiple common table expressions ([CommonTableExpression]).
///
/// These named queries can act as a "query local table" that are materialized during execution and
/// then can be used by the query prefixed with the WITH clause.
///
/// A WITH clause can contain multiple of these [CommonTableExpression]. (Except in the case of
/// recursive WITH query which can only contain one [CommonTableExpression]).
///
/// A [CommonTableExpression] is a name, column names and a query returning data for those columns.
///
/// Some databases (like sqlite) restrict the acceptable kinds of queries inside of the WITH clause
/// common table expressions. These databases only allow [SelectStatement]s to form a common table
/// expression.
///
/// Other databases like postgres allow modification queries (UPDATE, DELETE) inside of the WITH
/// clause but they have to return a table. (They must have a RETURNING clause).
///
/// tideorm-sql-builder doesn't check this or restrict the kind of [CommonTableExpression] that you can create
/// in rust. This means that you can put an UPDATE or DELETE queries into WITH clause and tideorm-sql-builder
/// will succeed in generating that kind of sql query but the execution inside the database will
/// fail because they are invalid.
///
/// It is your responsibility to ensure that the kind of WITH clause that you put together makes
/// sense and valid for that database that you are using.
///
/// NOTE that for recursive WITH queries (in sql: "WITH RECURSIVE") you can only have a
/// single [CommonTableExpression] inside of the WITH clause. That query must match certain
/// requirements:
///   * It is a query of UNION or UNION ALL of two queries.
///   * The first part of the query (the left side of the UNION) must be executable first in itself.
///     It must be non-recursive. (Cannot contain self reference)
///   * The self reference must appear in the right hand side of the UNION.
///   * The query can only have a single self-reference.
///   * Recursive data-modifying statements are not supported, but you can use the results of a
///     recursive SELECT query in a data-modifying statement. (like so: WITH RECURSIVE
///     cte_name(a,b,c,d) AS (SELECT ... UNION SELECT ... FROM ... JOIN cte_name ON ... WHERE ...)
///     DELETE FROM table WHERE table.a = cte_name.a)
///
/// It is mandatory to set the [Self::cte] and the [Self::query].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WithQuery {
    pub(crate) with_clause: WithClause,
    pub(crate) query: Option<Box<SubQueryStatement>>,
}

impl WithQuery {
    /// Constructs a new empty [WithQuery].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the whole [WithClause].
    pub fn with_clause(&mut self, with_clause: WithClause) -> &mut Self {
        self.with_clause = with_clause;
        self
    }

    /// Set the [WithClause::recursive]. See that method for more information.
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.with_clause.recursive = recursive;
        self
    }

    /// Add a [CommonTableExpression] to the with clause. See [WithClause::cte].
    pub fn cte(&mut self, cte: CommonTableExpression) -> &mut Self {
        self.with_clause.cte_expressions.push(cte);
        self
    }

    /// Set the query that you execute with the [WithClause].
    pub fn query<T>(&mut self, query: T) -> &mut Self
    where
        T: Into<SubQueryStatement>,
    {
        self.query = Some(Box::new(query.into()));
        self
    }
}

impl QueryStatementBuilder for WithQuery {
    fn build_collect_any_into(
        &self,
        query_builder: &impl QueryBuilder,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        functions::prepare_with_query(query_builder, self, sql)
    }
}

impl From<WithQuery> for SubQueryStatement {
    fn from(s: WithQuery) -> Self {
        Self::WithStatement(s)
    }
}

query_statement_writer!(WithQuery);
