#![warn(clippy::nursery)]
#![deny(unsafe_code)]
#![deny(
    missing_debug_implementations,
    clippy::print_stderr,
    clippy::print_stdout
)]
#![allow(
    clippy::derive_partial_eq_without_eq,
    clippy::option_if_let_else,
    clippy::redundant_pub_crate,
    clippy::use_self,
    clippy::missing_const_for_fn
)]
#![doc = include_str!("lib.md")]

pub mod backend;
pub mod column_type;
pub mod error;
pub mod expr;
pub mod extension;
pub mod func;
pub mod lexer;
pub mod prepare;
pub mod query;
pub mod types;
pub mod value;

pub use backend::*;
pub use column_type::*;
pub use error::RenderError;
pub use expr::*;
pub use func::*;
pub use prepare::*;
pub use query::*;
pub use types::*;
pub use value::*;

pub use sql_builder_derive::{Iden, IdenStatic};

#[cfg(test)]
#[path = "../tests/unit/builder_contracts_tests.rs"]
mod builder_contracts_tests;

#[cfg(test)]
#[path = "../tests/unit/render_errors_tests.rs"]
mod render_errors_tests;

#[cfg(test)]
#[path = "../tests/unit/quoted_name_tests.rs"]
mod quoted_name_tests;
