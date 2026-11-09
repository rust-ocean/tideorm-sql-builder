//! The `Iden` and `IdenStatic` derives, re-exported by `sql_builder`. Their
//! expansions name relative `sql_builder::` paths, so a user needs
//! `sql_builder` in scope.

mod case;
mod iden;

use proc_macro::TokenStream;

/// Implements `Iden` for an enum or a unit struct (see `sql_builder::Iden`).
#[proc_macro_derive(Iden, attributes(iden, method))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    iden::expand(input)
}

/// Implements `IdenStatic` for an enum or a unit struct (see `sql_builder::IdenStatic`).
#[proc_macro_derive(IdenStatic, attributes(iden, method))]
pub fn derive_iden_static(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    iden::iden_static::expand(input)
}
