//! For calling built-in Postgres SQL functions.

use crate::{PgDateTruncUnit, expr::*, func::*};

mod aggregate;
mod search;

/// Known Postgres-specific functions.
///
/// For all supported functions (including the standard ones), see [`Func`].
///
/// If something is not supported, you can use [`Func::Custom`].
#[derive(Debug, Clone, PartialEq)]
pub enum PgFunc {
    ToTsquery,
    ToTsvector,
    PhrasetoTsquery,
    PlaintoTsquery,
    WebsearchToTsquery,
    TsRank,
    TsRankCd,
    StartsWith,
    GenRandomUUID,
    JsonBuildObject,
    JsonAgg,
    ArrayAgg,
    DateTrunc,
    Any,
    Some,
    All,
    AdvisoryLock,
    AdvisoryLockShared,
    TryAdvisoryLock,
    TryAdvisoryLockShared,
    AdvisoryUnlock,
    AdvisoryUnlockShared,
    AdvisoryUnlockAll,
    AdvisoryXactLock,
    AdvisoryXactLockShared,
    TryAdvisoryXactLock,
    TryAdvisoryXactLockShared,
}

impl From<PgFunc> for Func {
    fn from(func: PgFunc) -> Self {
        Self::PgFunction(func)
    }
}
