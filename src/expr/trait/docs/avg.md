Express a `AVG` (average) function.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .expr(Expr::col(("character", "size_w")).avg())
    .from("character")
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT AVG(`character`.`size_w`) FROM `character`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT AVG("character"."size_w") FROM "character""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT AVG("character"."size_w") FROM "character""#
);
```
