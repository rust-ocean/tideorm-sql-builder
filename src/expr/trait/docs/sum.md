Express a `SUM` function.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .expr(Expr::col(("character", "size_w")).sum())
    .from("character")
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT SUM(`character`.`size_w`) FROM `character`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT SUM("character"."size_w") FROM "character""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT SUM("character"."size_w") FROM "character""#
);
```
