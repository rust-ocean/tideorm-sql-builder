Create any binary operation

# Examples
```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .cond_where(all![
        "size_w".into_column_ref().binary(BinOper::SmallerThan, 10),
        "size_w".into_column_ref().binary(BinOper::GreaterThan, "size_h".into_column_ref())
    ])
    .to_owned();
assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` < 10 AND `size_w` > `size_h`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" < 10 AND "size_w" > "size_h""#
);
```
