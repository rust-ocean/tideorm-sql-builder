Express a `COUNT` function with the DISTINCT modifier.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .expr(Expr::col(("character", "size_w")).count_distinct())
    .from("character")
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT COUNT(DISTINCT `character`.`size_w`) FROM `character`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT COUNT(DISTINCT "character"."size_w") FROM "character""#
);
```
