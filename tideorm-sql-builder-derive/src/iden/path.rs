//! The names the `Iden` derive reads in its attributes.

use std::fmt::{self, Display, Formatter};

/// An attribute name: `#[iden = ".."]`, `#[method = ".."]`, or one of the arguments of
/// `#[iden(..)]`: `rename = ".."`, `flatten` and `method = ".."`.
#[derive(Debug)]
pub enum IdenPath {
    Iden,
    Method,
    Rename,
    Flatten,
}

/// The name as written. It is also what lets `syn::Path::is_ident` take an `IdenPath`,
/// through `proc_macro2::Ident`'s comparison with any `AsRef<str>`.
impl AsRef<str> for IdenPath {
    fn as_ref(&self) -> &str {
        match self {
            Self::Iden => "iden",
            Self::Method => "method",
            Self::Rename => "rename",
            Self::Flatten => "flatten",
        }
    }
}

impl Display for IdenPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}
