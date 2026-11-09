//! The statements the builder renders; [`Query`] starts any of them.
//!
//! [`SelectStatement`], [`InsertStatement`], [`UpdateStatement`] and [`DeleteStatement`],
//! with `WITH` queries and the clauses they share: conditions, `CASE`, `ORDER BY`, window
//! frames, `ON CONFLICT` and `RETURNING`.

mod case;
mod condition;
mod delete;
mod insert;
mod on_conflict;
mod ordered;
mod returning;
mod select;
mod traits;
mod update;
mod window;
mod with;

pub use case::*;
pub use condition::*;
pub use delete::*;
pub use insert::*;
pub use on_conflict::*;
pub use ordered::*;
pub use returning::*;
pub use select::*;
pub use traits::*;
pub use update::*;
pub use window::*;
pub use with::*;

/// Where a statement starts: `Query::select()`, `Query::insert()` and so on.
#[derive(Debug, Clone)]
pub struct Query;

/// One of the four table statements.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum QueryStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
}

/// A statement nested in another: an expression's subquery or a derived table.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum SubQueryStatement {
    SelectStatement(SelectStatement),
    InsertStatement(InsertStatement),
    UpdateStatement(UpdateStatement),
    DeleteStatement(DeleteStatement),
    WithStatement(WithQuery),
}

impl Query {
    /// An empty [`SelectStatement`].
    pub fn select() -> SelectStatement {
        SelectStatement::new()
    }

    /// An empty [`InsertStatement`].
    pub fn insert() -> InsertStatement {
        InsertStatement::new()
    }

    /// An empty [`UpdateStatement`].
    pub fn update() -> UpdateStatement {
        UpdateStatement::new()
    }

    /// An empty [`DeleteStatement`].
    pub fn delete() -> DeleteStatement {
        DeleteStatement::new()
    }

    /// An empty [`WithClause`], for the common table expressions of a `WITH` query.
    pub fn with() -> WithClause {
        WithClause::new()
    }

    /// The builder of a `RETURNING` column list.
    pub fn returning() -> Returning {
        Returning::new()
    }
}
