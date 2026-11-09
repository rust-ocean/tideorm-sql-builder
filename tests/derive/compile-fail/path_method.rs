use sql_builder::Iden;

#[derive(Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[method]
    Creation,
}

fn main() {}
