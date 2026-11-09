Express an arithmetic addition operation.

# Examples

Adding literal values

```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .and_where(1.add(1).eq(2))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 + 1 = 2"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 + 1 = 2"#
);
```

Adding columns and values

```
use sql_builder::*;

let query = Query::select()
    .expr(Expr::col("size_w").add(1))
    .from("character")
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `size_w` + 1 FROM `character`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "size_w" + 1 FROM "character""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "size_w" + 1 FROM "character""#
);
```

Adding columns

```
use sql_builder::*;

let query = Query::select()
    .expr(Expr::col("size_w").add(Expr::col("size_h")))
    .from("character")
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `size_w` + `size_h` FROM `character`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "size_w" + "size_h" FROM "character""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "size_w" + "size_h" FROM "character""#
);
```
