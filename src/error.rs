//! Error types used in tideorm-sql-builder.

/// A statement that cannot be rendered as SQL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// The backend has no SQL for this, or the statement is incomplete.
    Unsupported(String),
    /// A numbered marker is zero, malformed, or too large to represent.
    InvalidIndex(String),
    /// A marker names an argument that was not supplied (one-based index).
    MissingValue { index: usize, count: usize },
    /// An argument has no corresponding marker (one-based index).
    UnusedValue { index: usize },
}

impl std::error::Error for RenderError {}
impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(message) => f.write_str(message),
            Self::InvalidIndex(index) => write!(f, "invalid parameter index: {index}"),
            Self::MissingValue { index, count } => {
                write!(f, "parameter {index} has no value ({count} supplied)")
            }
            Self::UnusedValue { index } => write!(f, "parameter value {index} is unused"),
        }
    }
}

/// Result type for tideorm-sql-builder
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Column and value vector having different length
    ColValNumMismatch { col_len: usize, val_len: usize },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ColValNumMismatch { col_len, val_len } => write!(
                f,
                "Columns and values length mismatch: {col_len} != {val_len}"
            ),
        }
    }
}
