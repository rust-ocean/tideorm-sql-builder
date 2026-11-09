Express a logical `OR` operation.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .and_where(false.or(true))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE FALSE OR TRUE"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE FALSE OR TRUE"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE FALSE OR TRUE"#
);
```
