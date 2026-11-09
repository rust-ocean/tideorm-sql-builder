use sql_builder::{Iden, IntoIden, MysqlQueryBuilder, PostgresQueryBuilder, QuotedBuilder};
use std::borrow::Cow;

#[derive(Iden)]
// Outer iden attributes overrides what's used for "Table"...
#[iden(rename = "user")]
enum Custom {
    Table,
    #[iden(rename = "my_id")]
    Id,
    #[iden(rename = "name")]
    FirstName,
    #[iden(rename = "surname")]
    LastName,
    // Custom casing if needed
    #[iden(rename = "EM`ail")]
    // the tuple value will be ignored
    Email(String),
    // Custom method
    #[iden(method = "custom_to_string")]
    Custom(String),
}

impl Custom {
    pub fn custom_to_string(&self) -> &str {
        match self {
            Self::Custom(custom) => custom,
            _ => panic!("not Custom::Custom"),
        }
    }
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = ["user", "my_id", "name", "surname", "EM`ail", ""];
    [
        Custom::Table,
        Custom::Id,
        Custom::FirstName,
        Custom::LastName,
        Custom::Email(String::new()),
        Custom::Custom(String::new()),
    ]
    .into_iter()
    .map(|var| var.to_string())
    .zip(expected)
    .for_each(|(iden, exp)| assert_eq!(iden, exp));

    let mut string = String::new();
    PostgresQueryBuilder.prepare_iden(&Custom::Email("".to_owned()).into_iden(), &mut string);
    assert_eq!(string, "\"EM`ail\"");

    let mut string = String::new();
    MysqlQueryBuilder.prepare_iden(&Custom::Email("".to_owned()).into_iden(), &mut string);
    assert_eq!(string, "`EM``ail`");

    assert!(matches!(Custom::FirstName.quoted(), Cow::Owned(_)));
}
