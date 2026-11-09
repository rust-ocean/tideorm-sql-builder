use sql_builder::{Iden, IdenStatic};

#[derive(Copy, Clone, IdenStatic)]
enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
}

#[derive(Copy, Clone, IdenStatic)]
enum UserStatic {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
}

fn main() {
    let expected = ["user", "id", "first_name", "last_name", "email"];
    [
        User::Table,
        User::Id,
        User::FirstName,
        User::LastName,
        User::Email,
    ]
    .into_iter()
    .zip(expected)
    .for_each(|(var, exp)| {
        assert_eq!(var.to_string(), exp);
        assert_eq!(var.as_str(), exp)
    });
}
