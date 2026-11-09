//! SQL that only one backend understands: PostgreSQL's and SQLite's own operators,
//! expressions and functions.

/// Declares a backend's binary operators: the enum, each variant's documentation led by
/// the SQL it renders as, and `sql()` giving that SQL to the backend's query builder.
macro_rules! binary_operators {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$doc:meta])* $variant:ident = $sql:literal,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = concat!("`", $sql, "`:")]
                $(#[$doc])*
                $variant,
            )+
        }

        impl $name {
            /// The operator as written in SQL.
            pub(crate) const fn sql(self) -> &'static str {
                match self {
                    $(Self::$variant => $sql,)+
                }
            }
        }
    };
}

pub mod postgres;

pub mod sqlite;
