use super::*;

impl Func {
    /// Call `UPPER` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::upper(Expr::col("character")))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT UPPER(`character`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT UPPER("character") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT UPPER("character") FROM "character""#
    /// );
    /// ```
    pub fn upper<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Upper).arg(expr)
    }

    /// Call `BIT_AND` function, this is not supported on SQLite.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::bit_and(Expr::col("font_size")))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT BIT_AND(`font_size`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT BIT_AND("font_size") FROM "character""#
    /// );
    /// ```
    pub fn bit_and<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::BitAnd).arg(expr)
    }

    /// Call `BIT_OR` function, this is not supported on SQLite.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::bit_or(Expr::col("font_size")))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT BIT_OR(`font_size`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT BIT_OR("font_size") FROM "character""#
    /// );
    /// ```
    pub fn bit_or<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::BitOr).arg(expr)
    }

    /// Call `ROUND` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(Func::round(5.654)).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT ROUND(5.654)"#);
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ROUND(5.654)"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT ROUND(5.654)"#
    /// );
    /// ```
    pub fn round<A>(expr: A) -> FunctionCall
    where
        A: Into<Expr>,
    {
        FunctionCall::new(Func::Round).arg(expr)
    }

    /// Call `ROUND` function with the precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::round_with_precision(5.654, 2))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT ROUND(5.654, 2)"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ROUND(5.654, 2)"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT ROUND(5.654, 2)"#
    /// );
    /// ```
    pub fn round_with_precision<A, B>(a: A, b: B) -> FunctionCall
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        FunctionCall::new(Func::Round).args([a.into(), b.into()])
    }

    /// Call `RANDOM` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(Func::random()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT RAND()"#);
    ///
    /// assert_eq!(query.to_string(PostgresQueryBuilder).unwrap(), r#"SELECT RANDOM()"#);
    ///
    /// assert_eq!(query.to_string(SqliteQueryBuilder).unwrap(), r#"SELECT RANDOM()"#);
    /// ```
    pub fn random() -> FunctionCall {
        FunctionCall::new(Func::Random)
    }

    /// Call `MD5` function, this is only available in Postgres and MySQL.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::md5(Expr::col(("character", "character"))))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MD5(`character`.`character`) FROM `character`"#
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MD5("character"."character") FROM "character""#
    /// );
    /// ```
    pub fn md5<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Md5).arg(expr)
    }
}
