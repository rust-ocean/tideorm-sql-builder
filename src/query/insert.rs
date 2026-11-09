mod build;
mod conflict;
mod values;

use crate::{
    DynIden, Expr, IntoColumnRef, IntoIden, IntoTableRef, OnConflict, QueryStatement,
    ReturningClause, SelectStatement, SubQueryStatement, TableRef, WithClause, WithQuery,
    error::{Error, Result},
};

/// Represents a value source that can be used in an insert query.
///
/// [`InsertValueSource`] is a node in the expression tree and can represent a raw value set
/// ('VALUES') or a select query.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InsertValueSource {
    Values(Vec<Vec<Expr>>),
    Select(Box<SelectStatement>),
}

/// Insert any new rows into an existing table
///
/// # Examples
///
/// ```
/// use sql_builder::*;
///
/// let query = Query::insert()
///     .into_table("glyph")
///     .columns(["aspect", "image"])
///     .values_panic([5.15.into(), "12A".into()])
///     .values_panic([4.21.into(), "123".into()])
///     .take();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(PostgresQueryBuilder).unwrap(),
///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// assert_eq!(
///     query.to_string(SqliteQueryBuilder).unwrap(),
///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
/// );
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct InsertStatement {
    pub(crate) replace: bool,
    pub(crate) table: Option<Box<TableRef>>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) source: Option<InsertValueSource>,
    pub(crate) on_conflict: Option<OnConflict>,
    pub(crate) returning: Option<ReturningClause>,
    pub(crate) default_values: Option<usize>,
    pub(crate) with: Option<WithClause>,
}

impl InsertStatement {
    /// Default rows that will actually render; explicit columns or a source override them.
    pub(crate) fn active_default_rows(&self) -> Option<usize> {
        self.default_values
            .filter(|count| *count != 0 && self.columns.is_empty() && self.source.is_none())
    }
}
