 TideORM's SQL builder, re-exported by the engine as `tideorm::orm::sql_builder`.

 ## Usage

 Table of Content

 1. Basics

     1. [Iden](#iden)
     1. [Expression](#expression)
     1. [Condition](#condition)
     1. [Statement Builders](#statement-builders)

 1. Query Statement

     1. [Query Select](#query-select)
     1. [Query Insert](#query-insert)
     1. [Query Update](#query-update)
     1. [Query Delete](#query-delete)

 1. Advanced
     1. [Aggregate Functions](#aggregate-functions)
     1. [Casting](#casting)
     1. [Custom Function](#custom-function)

 ## Motivation

 Why would you want to use a dynamic query builder?

 ### 1. Parameter bindings

 One of the headaches when using raw SQL is parameter binding. Here you can inject parameters
 right alongside the expression, and the $N sequencing will be handled for you. No more "off by one" errors!

 ```
 # use sql_builder::*;
 assert_eq!(
     Query::select()
         .expr(Expr::col("size_w").add(1).mul(2))
         .from("glyph")
         .and_where(Expr::col("image").like("A"))
         .and_where(Expr::col("id").is_in([3, 4, 5]))
         .build(PostgresQueryBuilder).unwrap(),
     (
         r#"SELECT ("size_w" + $1) * $2 FROM "glyph" WHERE "image" LIKE $3 AND "id" IN ($4, $5, $6)"#
             .to_owned(),
         Values(vec![
             1.into(),
             2.into(),
             "A".to_owned().into(),
             3.into(),
             4.into(),
             5.into(),
         ])
     )
 );
 ```

 If you need an "escape hatch" to construct complex queries, you can use custom expressions,
 and still have the benefit of sequentially-binded parameters.

 ```
 # use sql_builder::*;
 assert_eq!(
     Query::select()
         .columns(["size_w", "size_h"])
         .from("character")
         .and_where(Expr::col("id").eq(1)) // this is $1
         // custom expressions only need to define local parameter sequence.
         // its global sequence will be re-written.
         // here, we flip the order of $2 & $1 to make it look tricker!
         .and_where(Expr::cust_with_values(r#""size_w" = $2 * $1"#, [3, 2], &PostgresQueryBuilder).unwrap())
         .and_where(Expr::col("size_h").gt(4)) // this is $N?
         .build(PostgresQueryBuilder).unwrap(),
     (
         r#"SELECT "size_w", "size_h" FROM "character" WHERE "id" = $1 AND ("size_w" = $2 * $3) AND "size_h" > $4"#
             .to_owned(),
         Values(vec![1.into(), 2.into(), 3.into(), 4.into()])
     )
 );
 ```

 ### 2. Dynamic query

 You can construct the query at runtime based on user inputs with a fluent interface,
 so you don't have to append `WHERE` or `AND` conditionally.

 ```
 # use sql_builder::*;
 fn query(a: Option<i32>, b: Option<char>) -> SelectStatement {
     Query::select()
         .column("id")
         .from("character")
         .apply_if(a, |q, v| {
             q.and_where(Expr::col("font_id").eq(v));
         })
         .apply_if(b, |q, v| {
             q.and_where(Expr::col("ascii").like(v));
         })
         .take()
 }

 assert_eq!(
     query(Some(5), Some('A')).to_string(MysqlQueryBuilder).unwrap(),
     "SELECT `id` FROM `character` WHERE `font_id` = 5 AND `ascii` LIKE 'A'"
 );
 assert_eq!(
     query(Some(5), None).to_string(MysqlQueryBuilder).unwrap(),
     "SELECT `id` FROM `character` WHERE `font_id` = 5"
 );
 assert_eq!(
     query(None, None).to_string(MysqlQueryBuilder).unwrap(),
     "SELECT `id` FROM `character`"
 );
 ```

 Conditions can be arbitrarily complex, thanks to the builder's internal AST:

 ```
 # use sql_builder::*;
 assert_eq!(
     Query::select()
         .column("id")
         .from("glyph")
         .cond_where(
             Cond::any()
                 .add(
                     Cond::all()
                         .add(Expr::col("aspect").is_null())
                         .add(Expr::col("image").is_null())
                 )
                 .add(
                     Cond::all()
                         .add(Expr::col("aspect").is_in([3, 4]))
                         .add(Expr::col("image").like("A%"))
                 )
         )
         .to_string(PostgresQueryBuilder).unwrap(),
     [
         r#"SELECT "id" FROM "glyph""#,
         r#"WHERE"#,
         r#"("aspect" IS NULL AND "image" IS NULL)"#,
         r#"OR"#,
         r#"("aspect" IN (3, 4) AND "image" LIKE 'A%')"#,
     ]
     .join(" ")
 );
 ```

 There is no superfluous parentheses `((((` cluttering the query, because the builder respects
 operator precedence when injecting them.

 ### 3. Cross database support

 You can target multiple database backends while maintaining a single source of query logic.

 ```
 # use sql_builder::*;
 let query = Query::insert()
     .into_table("glyph")
     .columns(["aspect", "image"])
     .values_panic([
         2.into(),
         3.into(),
     ])
     .on_conflict(
         OnConflict::column("id")
             .update_columns(["aspect", "image"])
             .to_owned(),
     )
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, 3) ON DUPLICATE KEY UPDATE `aspect` = VALUES(`aspect`), `image` = VALUES(`image`)"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, 3) ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#
 );
 ```

 ## Basics

 ### Iden

 `Iden` is a trait for identifiers used in any query statement.

 Commonly implemented by Enum where each Enum represents a table found in a database,
 and its variants include table name and column name.

 You can use the `Iden` derive macro to implement it.

 ```rust
 # use sql_builder::Iden;
 #[derive(Iden)]
 enum Character {
     Table,
     Id,
     FontId,
     FontSize,
 }

 assert_eq!(Character::Table.to_string(), "character");
 assert_eq!(Character::Id.to_string(), "id");
 assert_eq!(Character::FontId.to_string(), "font_id");
 assert_eq!(Character::FontSize.to_string(), "font_size");

 #[derive(Iden)]
 struct Glyph;
 assert_eq!(Glyph.to_string(), "glyph");
 ```

 ### Expression

 Use [`Expr`] constructors and [`ExprTrait`] methods
 to construct `SELECT`, `JOIN`, `WHERE` and `HAVING` expression in query.

 ```rust
 # use sql_builder::*;
 assert_eq!(
     Query::select()
         .column("char_code")
         .from("character")
         .and_where(
             Expr::col("size_w")
                 .add(1)
                 .mul(2)
                 .eq(Expr::col("size_h").div(2).sub(1))
         )
         .and_where(
             Expr::col("size_w").in_subquery(
                 Query::select()
                     .expr(Expr::cust_with_values("ln($1 ^ $2)", [2.4, 1.2], &PostgresQueryBuilder).unwrap())
                     .take()
             )
         )
         .and_where(
             Expr::col("char_code")
                 .like("D")
                 .and(Expr::col("char_code").like("E"))
         )
         .to_string(PostgresQueryBuilder).unwrap(),
     [
         r#"SELECT "char_code" FROM "character""#,
         r#"WHERE ("size_w" + 1) * 2 = ("size_h" / 2) - 1"#,
         r#"AND "size_w" IN (SELECT ln(2.4 ^ 1.2))"#,
         r#"AND ("char_code" LIKE 'D' AND "char_code" LIKE 'E')"#,
     ]
     .join(" ")
 );
 ```

 ### Condition

 If you have complex conditions to express, you can use the [`Condition`] builder,
 usable for [`ConditionalStatement::cond_where`] and [`SelectStatement::cond_having`].

 ```
 # use sql_builder::*;
 assert_eq!(
     Query::select()
         .column("id")
         .from("glyph")
         .cond_where(
             Cond::any()
                 .add(
                     Cond::all()
                         .add(Expr::col("aspect").is_null())
                         .add(Expr::col("image").is_null())
                 )
                 .add(
                     Cond::all()
                         .add(Expr::col("aspect").is_in([3, 4]))
                         .add(Expr::col("image").like("A%"))
                 )
         )
         .to_string(PostgresQueryBuilder).unwrap(),
     [
         r#"SELECT "id" FROM "glyph""#,
         r#"WHERE"#,
         r#"("aspect" IS NULL AND "image" IS NULL)"#,
         r#"OR"#,
         r#"("aspect" IN (3, 4) AND "image" LIKE 'A%')"#,
     ]
     .join(" ")
 );
 ```

 There is also the [`any!`] and [`all!`] macro at your convenience:

 ```
 # use sql_builder::*;
 Query::select().cond_where(any![
     Expr::col("aspect").is_in([3, 4]),
     all![
         Expr::col("aspect").is_null(),
         Expr::col("image").like("A%")
     ]
 ]);
 ```

 ### Statement Builders

 Query statements are serialized into SQL with [`QueryStatementBuilder`]:

 ```rust
 # use sql_builder::{*};
 # trait ExampleQueryBuilder {
 fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values);

 fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String;
 # }
 ```

 `build` builds a SQL statement as string and parameters to be passed to the database driver
 through the binary protocol. This is the preferred way as it has less overhead and is more secure.

 `to_string` builds a SQL statement as string with parameters injected. This is good for testing
 and debugging.

 ## Query Statement

 ### Query Select

 ```rust
 # use sql_builder::*;
 let query = Query::select()
     .column("char_code")
     .column(("font", "name"))
     .from("character")
     .left_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
     .and_where(Expr::col("size_w").is_in([3, 4]))
     .and_where(Expr::col("char_code").like("A%"))
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"SELECT `char_code`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `char_code` LIKE 'A%'"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"SELECT "char_code", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "char_code" LIKE 'A%'"#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"SELECT "char_code", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "char_code" LIKE 'A%'"#
 );
 ```

 ### Query Insert

 ```rust
 # use sql_builder::*;
 let query = Query::insert()
     .into_table("glyph")
     .columns(["aspect", "image"])
     .values_panic([5.15.into(), "12A".into()])
     .values_panic([4.21.into(), "123".into()])
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
 );
 ```

 ### Query Update

 ```rust
 # use sql_builder::*;
 let query = Query::update()
     .table("glyph")
     .values([("aspect", 1.23.into()), ("image", "123".into())])
     .and_where(Expr::col("id").eq(1))
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"UPDATE `glyph` SET `aspect` = 1.23, `image` = '123' WHERE `id` = 1"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
 );
 ```

 ### Query Delete

 ```rust
 # use sql_builder::*;
 let query = Query::delete()
     .from_table("glyph")
     .cond_where(
         Cond::any()
             .add(Expr::col("id").lt(1))
             .add(Expr::col("id").gt(10)),
     )
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"DELETE FROM `glyph` WHERE `id` < 1 OR `id` > 10"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
 );
 ```

 ## Advanced

 ### Aggregate Functions

 `max`, `min`, `sum`, `avg`, `count` etc

 ```rust
 # use sql_builder::*;
 let query = Query::select()
     .expr(Func::sum(Expr::col(("character", "size_h"))))
     .from("character")
     .to_owned();
 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"SELECT SUM(`character`.`size_h`) FROM `character`"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"SELECT SUM("character"."size_h") FROM "character""#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"SELECT SUM("character"."size_h") FROM "character""#
 );
 ```

 ### Casting

 ```rust
 # use sql_builder::*;
 let query = Query::select()
     .expr(Func::cast_as("hello", "MyType"))
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"SELECT CAST('hello' AS MyType)"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"SELECT CAST('hello' AS MyType)"#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"SELECT CAST('hello' AS MyType)"#
 );
 ```

 ### Custom Function

 ```rust
 # use sql_builder::*;
 struct MyFunction;

 impl Iden for MyFunction {
     fn unquoted(&self) -> &str {
         "MY_FUNCTION"
     }
 }

 let query = Query::select()
     .expr(Func::cust(MyFunction).arg(Expr::val("hello")))
     .to_owned();

 assert_eq!(
     query.to_string(MysqlQueryBuilder).unwrap(),
     r#"SELECT MY_FUNCTION('hello')"#
 );
 assert_eq!(
     query.to_string(PostgresQueryBuilder).unwrap(),
     r#"SELECT MY_FUNCTION('hello')"#
 );
 assert_eq!(
     query.to_string(SqliteQueryBuilder).unwrap(),
     r#"SELECT MY_FUNCTION('hello')"#
 );
 ```
