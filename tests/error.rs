use sql_builder::{error::*, *};

#[test]
fn insert_values_1() {
    let mut insert = Query::insert();
    let result = insert
        .into_table("glyph")
        .columns(["image", "aspect"])
        .values([String::from("").into()]);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        Error::ColValNumMismatch {
            col_len: 2,
            val_len: 1,
        }
    );
}
