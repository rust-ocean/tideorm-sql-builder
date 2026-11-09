//! SQLite's own operators and expressions.

pub use expr::SqliteExpr;

use crate::types::BinOper;

mod expr;

binary_operators! {
    /// The binary operators only SQLite has; [`BinOper`] holds them alongside the standard
    /// ones.
    pub enum SqliteBinOper {
        /// matches a Unix file-name pattern, case-sensitively.
        Glob = "GLOB",
        /// full-text search, answered by the table or function that defines it.
        Match = "MATCH",
        /// a JSON object member or array element, as JSON text.
        GetJsonField = "->",
        /// a JSON object member or array element, as an SQL value.
        CastJsonField = "->>",
    }
}

impl From<SqliteBinOper> for BinOper {
    fn from(o: SqliteBinOper) -> Self {
        Self::SqliteOperator(o)
    }
}
