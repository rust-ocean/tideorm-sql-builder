use std::fmt::Display;

use crate::{
    BoundSql, ColumnRef, ConditionHolder, DynIden, Expr, FunctionCall, IntoColumnRef,
    IntoCondition, IntoIden, IntoTableRef, IntoValueTuple, JoinOn, JoinType, OrderExpr,
    QueryStatement, SubQueryStatement, TableRef, Value, ValueTuple, WindowStatement, WithClause,
    WithQuery,
};

mod build;
mod construction;
mod grouping;
mod join_custom;
mod joins;
mod modifiers;
mod projection;
mod sources;

/// Select rows from an existing table
///
/// # Examples
///
/// ```
/// use sql_builder::*;
///
/// let query = Query::select()
///     .column("character")
///     .column(("font", "name"))
///     .from("character")
///     .left_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
///     .and_where(Expr::col("size_w").is_in([3, 4]))
///     .and_where(Expr::col("character").like("A%"))
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder).unwrap(),
///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder).unwrap(),
///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SelectStatement {
    pub(crate) distinct: Option<SelectDistinct>,
    pub(crate) selects: Vec<SelectExpr>,
    pub(crate) into: Option<SelectInto>,
    pub(crate) from: Vec<TableRef>,
    pub(crate) join: Vec<JoinExpr>,
    pub(crate) r#where: ConditionHolder,
    pub(crate) groups: Vec<Expr>,
    pub(crate) having: ConditionHolder,
    pub(crate) unions: Vec<(UnionType, SelectStatement)>,
    pub(crate) orders: Vec<OrderExpr>,
    pub(crate) limit: Option<Value>,
    pub(crate) offset: Option<Value>,
    pub(crate) lock: Option<LockClause>,
    pub(crate) window: Option<(DynIden, WindowStatement)>,
    pub(crate) with: Option<Box<WithClause>>,
    pub(crate) custom: Option<BoundSql>,
}

/// List of distinct keywords that can be used in select statement
#[derive(Debug, Clone, PartialEq)]
pub enum SelectDistinct {
    All,
    Distinct,
    DistinctRow,
    DistinctOn(Vec<ColumnRef>),
}

/// Window type in [`SelectExpr`]
#[derive(Debug, Clone, PartialEq)]
pub enum WindowSelectType {
    /// Name in [`SelectStatement`]
    Name(DynIden),
    /// Inline query in [`SelectExpr`]
    Query(WindowStatement),
}

/// Select expression used in select statement
#[derive(Debug, Clone, PartialEq)]
pub struct SelectExpr {
    pub expr: Expr,
    pub alias: Option<DynIden>,
    pub window: Option<WindowSelectType>,
}

/// The table type of target table of SELECT INTO clause
#[derive(Debug, Clone, PartialEq)]
pub enum SelectIntoTableModifier {
    Temporary,
    Unlogged,
}

/// The INTO clause in SELECT statement
#[derive(Debug, Clone, PartialEq)]
pub struct SelectInto {
    pub target_table: DynIden,
    pub target_table_modifier: Option<SelectIntoTableModifier>,
}

/// Join expression used in select statement
#[derive(Debug, Clone, PartialEq)]
pub struct JoinExpr {
    pub join: JoinType,
    pub table: Box<TableRef>,
    pub on: Option<JoinOn>,
    pub lateral: bool,
}

/// List of lock types that can be used in select statement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    /// Exclusive lock
    Update,
    /// PostgreSQL only; MySQL refuses to render it
    NoKeyUpdate,
    /// Shared lock
    Share,
    /// PostgreSQL only; MySQL refuses to render it
    KeyShare,
}

/// List of lock behavior can be used in select statement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockBehavior {
    Nowait,
    SkipLocked,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LockClause {
    pub(crate) r#type: LockType,
    pub(crate) tables: Vec<TableRef>,
    pub(crate) behavior: Option<LockBehavior>,
}

/// List of union types that can be used in union clause
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnionType {
    Intersect,
    Distinct,
    Except,
    All,
}

impl<T> From<T> for SelectExpr
where
    T: Into<Expr>,
{
    fn from(expr: T) -> Self {
        SelectExpr {
            expr: expr.into(),
            alias: None,
            window: None,
        }
    }
}

impl Display for SelectIntoTableModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectIntoTableModifier::Temporary => f.write_str("TEMPORARY"),
            SelectIntoTableModifier::Unlogged => f.write_str("UNLOGGED"),
        }
    }
}

impl SelectInto {
    /// Set the target table of SELECT INTO clause
    pub fn table<T>(t: T) -> Self
    where
        T: IntoIden,
    {
        Self {
            target_table: t.into_iden(),
            target_table_modifier: None,
        }
    }

    /// Set a modifier for the target table like Temporary or unlogged
    pub fn modifier(mut self, modifier: SelectIntoTableModifier) -> Self {
        self.target_table_modifier = Some(modifier);
        self
    }
}

impl From<WindowStatement> for WindowSelectType {
    fn from(stmt: WindowStatement) -> Self {
        Self::Query(stmt)
    }
}

impl<T: IntoIden> From<T> for WindowSelectType {
    fn from(iden: T) -> Self {
        Self::Name(iden.into_iden())
    }
}

/// Extension methods for building a [`SelectExpr`] from an expression.
///
/// This makes it ergonomic to attach select-specific modifiers (like `AS` and `OVER`) and pass the
/// result into [`SelectStatement::expr`].
///
/// # Examples
///
/// ```
/// use sql_builder::*;
///
/// let query = Query::select()
///     .from("character")
///     .expr(
///         Expr::col("character")
///             .max()
///             .over(WindowStatement::partition_by("font_size"))
///             .alias("C"),
///     )
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"SELECT MAX(`character`) OVER ( PARTITION BY `font_size` ) AS `C` FROM `character`"#
/// );
/// ```
pub trait SelectExprTrait: Sized {
    fn alias<A>(self, alias: A) -> SelectExpr
    where
        A: IntoIden;

    fn over(self, over_expr: impl Into<WindowSelectType>) -> SelectExpr;
}

impl SelectExprTrait for SelectExpr {
    fn alias<A>(mut self, alias: A) -> SelectExpr
    where
        A: IntoIden,
    {
        self.alias = Some(alias.into_iden());
        self
    }

    fn over(mut self, over_expr: impl Into<WindowSelectType>) -> SelectExpr {
        self.window = Some(over_expr.into());
        self
    }
}

impl<T> SelectExprTrait for T
where
    T: Into<Expr>,
{
    fn alias<A>(self, alias: A) -> SelectExpr
    where
        A: IntoIden,
    {
        SelectExpr::from(self).alias(alias)
    }

    fn over(self, over_expr: impl Into<WindowSelectType>) -> SelectExpr {
        SelectExpr::from(self).over(over_expr)
    }
}
