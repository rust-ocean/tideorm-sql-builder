mod attr;
mod error;
pub mod iden_static;
mod path;
mod write_arm;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::convert::{TryFrom, TryInto};

use crate::case::to_snake_case;
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, Fields, Variant};

use attr::IdenAttr;
use error::ErrorMsg;
use path::IdenPath;
use write_arm::IdenVariant;

struct DeriveIden;
struct DeriveIdenStatic;

pub fn expand(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = input;

    let table_name = match get_table_name(&ident, attrs) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let variants = match data {
        syn::Data::Enum(DataEnum { variants, .. }) => variants,
        syn::Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => return impl_iden_for_unit_struct(&ident, &table_name).into(),
        _ => {
            return quote_spanned! {
                ident.span() => compile_error!("you can only derive Iden on enums or unit structs");
            }
            .into();
        }
    };

    if variants.is_empty() {
        return TokenStream::new();
    }

    impl_iden_for_enum(&ident, &table_name, variants.iter()).into()
}

fn impl_iden_for_unit_struct(
    ident: &proc_macro2::Ident,
    table_name: &str,
) -> proc_macro2::TokenStream {
    if is_static_iden(table_name) {
        quote! {
            impl sql_builder::Iden for #ident {
                fn quoted(&self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(#table_name)
                }

                fn unquoted(&self) -> &str {
                    #table_name
                }
            }
        }
    } else {
        quote! {
            impl sql_builder::Iden for #ident {
                fn unquoted(&self) -> &str {
                    #table_name
                }
            }
        }
    }
}

fn impl_iden_for_enum<'a, T>(
    ident: &proc_macro2::Ident,
    table_name: &str,
    variants: T,
) -> proc_macro2::TokenStream
where
    T: Iterator<Item = &'a Variant>,
{
    let mut is_all_static_iden = true;

    let match_arms = match variants
        .map(|v| {
            let v = IdenVariant::<DeriveIden>::try_from((table_name, v))?;
            is_all_static_iden &= v.is_static_iden();
            Ok(v)
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    if is_all_static_iden {
        quote! {
            impl sql_builder::Iden for #ident {
                fn quoted(&self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(match self {
                        #(#match_arms),*
                    })
                }

                fn unquoted(&self) -> &str {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    } else {
        quote! {
            impl sql_builder::Iden for #ident {
                fn unquoted(&self) -> &str {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    }
}

fn get_table_name(ident: &proc_macro2::Ident, attrs: Vec<Attribute>) -> Result<String, syn::Error> {
    let table_name = match find_attr(&attrs) {
        Some(att) => match att.try_into()? {
            IdenAttr::Rename(lit) => lit,
            _ => return Err(syn::Error::new_spanned(att, ErrorMsg::ContainerAttr)),
        },
        None => to_snake_case(&ident.to_string()),
    };
    Ok(table_name)
}

fn find_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| {
        attr.path().is_ident(&IdenPath::Iden) || attr.path().is_ident(&IdenPath::Method)
    })
}

pub fn is_static_iden(name: &str) -> bool {
    name.chars()
        .take(1)
        .all(|c| c == '_' || c.is_ascii_alphabetic())
        && name.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
}
