Express a logical `AND` operation.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .cond_where(any![
        Expr::col("size_w").eq(1).and(Expr::col("size_h").eq(2)),
        Expr::col("size_w").eq(3).and(Expr::col("size_h").eq(4)),
    ])
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w` = 1 AND `size_h` = 2) OR (`size_w` = 3 AND `size_h` = 4)"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 AND "size_h" = 2) OR ("size_w" = 3 AND "size_h" = 4)"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" = 1 AND "size_h" = 2) OR ("size_w" = 3 AND "size_h" = 4)"#
);
```
