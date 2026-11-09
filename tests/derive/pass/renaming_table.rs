use sql_builder::Iden;

#[derive(Copy, Clone, Iden)]
enum Something {
    // ...the Table can also be overwritten like this
    #[iden = "something_else"]
    Table,
    Id,
    AssetName,
    UserId,
}

fn main() {
    let expected = ["something_else", "id", "asset_name", "user_id"];
    [
        Something::Table,
        Something::Id,
        Something::AssetName,
        Something::UserId,
    ]
    .into_iter()
    .zip(expected)
    .for_each(|(var, exp)| {
        assert_eq!(var.to_string(), exp);
    })
}
