//! PostgreSQL's own operators, expressions and functions.

pub use expr::*;
pub use func::*;

use crate::types::BinOper;

pub(crate) mod expr;
pub(crate) mod func;
pub(crate) mod interval;

binary_operators! {
    /// The binary operators only PostgreSQL has; [`BinOper`] holds them alongside the
    /// standard ones.
    ///
    /// The `pg_trgm` ones need that extension installed
    /// (<https://www.postgresql.org/docs/current/pgtrgm.html>).
    pub enum PgBinOper {
        /// `LIKE` ignoring case.
        ILike = "ILIKE",
        /// `NOT LIKE` ignoring case.
        NotILike = "NOT ILIKE",
        /// a `tsvector` matches a `tsquery` (full-text search).
        Matches = "@@",
        /// the left array or `jsonb` value contains the right one.
        Contains = "@>",
        /// the left array or `jsonb` value is contained in the right one.
        Contained = "<@",
        /// joins two strings, arrays or `jsonb` values.
        Concatenate = "||",
        /// the two arrays share an element.
        Overlap = "&&",
        /// the strings' trigram similarity passes the threshold (`pg_trgm`).
        Similarity = "%",
        /// the left string's word similarity to the right one passes the threshold
        /// (`pg_trgm`).
        WordSimilarity = "<%",
        /// as `WordSimilarity`, but counting whole words only (`pg_trgm`).
        StrictWordSimilarity = "<<%",
        /// one minus the strings' similarity (`pg_trgm`).
        SimilarityDistance = "<->",
        /// one minus their word similarity (`pg_trgm`).
        WordSimilarityDistance = "<<->",
        /// one minus their strict word similarity (`pg_trgm`).
        StrictWordSimilarityDistance = "<<<->",
        /// a JSON object member or array element, as JSON.
        GetJsonField = "->",
        /// a JSON object member or array element, as text.
        CastJsonField = "->>",
        /// matches a POSIX regular expression, case-sensitively.
        Regex = "~",
        /// matches a POSIX regular expression, ignoring case.
        RegexCaseInsensitive = "~*",
    }
}

impl From<PgBinOper> for BinOper {
    fn from(o: PgBinOper) -> Self {
        Self::PgOperator(o)
    }
}
