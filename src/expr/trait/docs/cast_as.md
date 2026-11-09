Express a `CAST AS` expression.

# Examples

```
use sql_builder::*;

let query = Query::select().expr("1".cast_as("integer")).to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder).unwrap(),
    r#"SELECT CAST('1' AS integer)"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder).unwrap(),
    r#"SELECT CAST('1' AS integer)"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder).unwrap(),
    r#"SELECT CAST('1' AS integer)"#
);
```
