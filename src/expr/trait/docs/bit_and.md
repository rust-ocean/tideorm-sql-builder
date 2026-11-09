Express a bitwise AND operation.

# Examples

```
use sql_builder::*;

let query = Query::select().expr(1.bit_and(2).eq(3)).to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT (1 & 2) = 3"#
);

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .and_where(1.bit_and(1).eq(1))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (1 & 1) = 1"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 & 1) = 1"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE (1 & 1) = 1"#
);
```
