use syn::Ident;

use super::path::IdenPath;

#[derive(Debug)]
pub enum ErrorMsg {
    ContainerAttr,
    WrongLiteral,
    WrongNamedValueFormat(IdenPath, IdenPath),
    WrongListFormat,
    WrongAttributeFormat,
    UnsupportedKeyword(Ident),
    UnsupportedFlattenTarget,
}

impl std::fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContainerAttr => f.write_str(
                "Only the attributes `#[iden = \"name\"]` or `#[iden(rename = \"name\") are supported in this position",
            ),
            Self::WrongLiteral => f.write_str("Must be a string literal"),
            Self::WrongNamedValueFormat(first, second) => write!(
                f,
                "The method attribute only supports the `#[{first} = \"name\"]` or `#[iden({second} = \"name\")]` formats"
            ),
            Self::WrongListFormat => f.write_str(
                "Must one of the following attributes: `flatten`, `rename` or `method`",
            ),
            Self::WrongAttributeFormat => f.write_str(
                "The iden attribute supports only the formats `#[iden = \"name\"]` or `#[iden(<ATTRIBUTE>)]` where ATTRIBUTE is either `flatten`, `rename` or `method`",
            ),
            Self::UnsupportedKeyword(ident) => write!(f, "{ident} is not a supported keyword"),
            Self::UnsupportedFlattenTarget => {
                f.write_str("Must have a single field is supported for flattenning")
            }
        }
    }
}

impl std::error::Error for ErrorMsg {}
