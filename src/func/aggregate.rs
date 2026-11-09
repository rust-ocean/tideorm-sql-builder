use super::*;

impl Func {
    /// Call a custom function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// #[derive(Iden)]
    /// #[iden = "MY_FUNCTION"]
    /// struct MyFunction;
    ///
    /// let query = Query::select()
    ///     .expr(Func::cust(MyFunction).arg("hello"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MY_FUNCTION('hello')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MY_FUNCTION('hello')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MY_FUNCTION('hello')"#
    /// );
    /// ```
    pub fn cust<T>(func: T) -> FunctionCall
    where
        T: IntoIden,
    {
        FunctionCall::new(Func::Custom(func.into_iden()))
    }

    /// Call `MAX` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::max(Expr::col(("character", "size_w"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MAX(`character`.`size_w`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character"."size_w") FROM "character""#
    /// );
    /// ```
    pub fn max<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Max).arg(expr)
    }

    /// Call `MIN` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::min(Expr::col(("character", "size_h"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MIN(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MIN("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MIN("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn min<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Min).arg(expr)
    }

    /// Call `SUM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::sum(Expr::col(("character", "size_h"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT SUM(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT SUM("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT SUM("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn sum<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Sum).arg(expr)
    }

    /// Call `AVG` function for average.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::avg(Expr::col(("character", "size_h"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT AVG(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT AVG("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT AVG("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn avg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Avg).arg(expr)
    }

    /// Call `ABS` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::abs(Expr::col(("character", "size_h"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT ABS(`character`.`size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ABS("character"."size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT ABS("character"."size_h") FROM "character""#
    /// );
    /// ```
    pub fn abs<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Abs).arg(expr)
    }

    /// Call `COUNT` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::count(Expr::col(("character", "id"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT(`character`.`id`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT("character"."id") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT("character"."id") FROM "character""#
    /// );
    /// ```
    pub fn count<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Count).arg(expr)
    }

    /// Call `COUNT` function with the `DISTINCT` modifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::count_distinct(Expr::col(("character", "id"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT(DISTINCT `character`.`id`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT(DISTINCT "character"."id") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT(DISTINCT "character"."id") FROM "character""#
    /// );
    /// ```
    pub fn count_distinct<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Count).arg_with(expr, FuncArgMod { distinct: true })
    }

    /// Call `CHAR_LENGTH` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::char_length(Expr::col(("character", "character"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT CHAR_LENGTH(`character`.`character`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CHAR_LENGTH("character"."character") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT LENGTH("character"."character") FROM "character""#
    /// );
    /// ```
    pub fn char_length<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::CharLength).arg(expr)
    }

    /// Call `GREATEST` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::greatest([
    ///         Expr::col("size_w").into(),
    ///         Expr::col("size_h").into(),
    ///     ]))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT GREATEST(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT GREATEST("size_w", "size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("size_w", "size_h") FROM "character""#
    /// );
    /// ```
    pub fn greatest<I>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = Expr>,
    {
        FunctionCall::new(Func::Greatest).args(args)
    }

    /// Call `LEAST` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::least([
    ///         Expr::col("size_w").into(),
    ///         Expr::col("size_h").into(),
    ///     ]))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT LEAST(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT LEAST("size_w", "size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MIN("size_w", "size_h") FROM "character""#
    /// );
    /// ```
    pub fn least<I>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = Expr>,
    {
        FunctionCall::new(Func::Least).args(args)
    }
}
