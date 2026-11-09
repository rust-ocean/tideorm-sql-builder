Express a equal expression between two table columns,
you will mainly use this to relate identical value between two table columns.

# Examples

```
use sql_builder::*;

let query = Query::select()
    .columns(["character", "size_w", "size_h"])
    .from("character")
    .and_where(("character", "font_id").into_column_ref().equals(("font", "id")))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`font_id` = `font`.`id`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."font_id" = "font"."id""#
);
```
