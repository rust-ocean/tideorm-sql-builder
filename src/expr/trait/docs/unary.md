Apply any unary operator to the expression.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .and_where(Expr::col(("character", "size_w")).is_null().unary(UnOper::Not))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE NOT `character`.`size_w` IS NULL"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE NOT "character"."size_w" IS NULL"#
);
```
