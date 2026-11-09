use sql_builder::Iden;

#[derive(Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden]
    Creation,
}

fn main() {}
