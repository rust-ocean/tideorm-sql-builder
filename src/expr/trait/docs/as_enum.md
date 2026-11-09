Express a `AS enum` expression.

# Examples

```
use sql_builder::*;

let query = Query::insert()
    .into_table("character")
    .columns(["font_size"])
    .values_panic(["large".as_enum("FontSizeEnum")])
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"INSERT INTO `character` (`font_size`) VALUES ('large')"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"INSERT INTO "character" ("font_size") VALUES (CAST('large' AS "FontSizeEnum"))"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"INSERT INTO "character" ("font_size") VALUES ('large')"#
);

let query = Query::select()
    .expr(Expr::col("font_size").as_enum("FontSizeEnum[]"))
    .from("character")
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT CAST("font_size" AS "FontSizeEnum"[]) FROM "character""#
);
```
