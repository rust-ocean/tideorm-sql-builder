Express a not equal (`<>`) expression.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    // Sometimes, you'll have to qualify the call because of conflicting std traits.
    .and_where(ExprTrait::ne("Morning", "Good"))
    .and_where("id".into_column_ref().ne(1))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 'Morning' <> 'Good' AND `id` <> 1"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 'Morning' <> 'Good' AND "id" <> 1"#
);
```
