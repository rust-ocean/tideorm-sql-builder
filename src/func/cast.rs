use super::*;

impl Func {
    /// Call `IF NULL` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::if_null(
    ///         Expr::col("size_w"),
    ///         Expr::col("size_h"),
    ///     ))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT IFNULL(`size_w`, `size_h`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT COALESCE("size_w", "size_h") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT IFNULL("size_w", "size_h") FROM "character""#
    /// );
    /// ```
    pub fn if_null<A, B>(a: A, b: B) -> FunctionCall
    where
        A: Into<Expr>,
        B: Into<Expr>,
    {
        FunctionCall::new(Func::IfNull).args([a.into(), b.into()])
    }

    /// Call `CAST` function with a custom type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as("hello", "MyType"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS MyType)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS MyType)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS MyType)"#
    /// );
    /// ```
    pub fn cast_as<V, I>(expr: V, iden: I) -> FunctionCall
    where
        V: Into<Expr>,
        I: IntoIden,
    {
        let expr: Expr = expr.into();
        FunctionCall::new(Func::Cast).arg(expr.binary(BinOper::As, Expr::cust(iden.into_iden().0)))
    }

    /// Call `CAST` function with a case-sensitive custom type.
    ///
    /// Type can be qualified with a schema name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as_quoted("hello", "MyType"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS `MyType`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS "MyType")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS "MyType")"#
    /// );
    ///
    /// // Also works with a schema-qualified type name:
    ///
    /// let query = Query::select()
    ///     .expr(Func::cast_as_quoted("hello", ("MySchema", "MyType")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS `MySchema`.`MyType`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS "MySchema"."MyType")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT CAST('hello' AS "MySchema"."MyType")"#
    /// );
    /// ```
    pub fn cast_as_quoted<V, I>(expr: V, r#type: I) -> FunctionCall
    where
        V: Into<Expr>,
        I: Into<TypeRef>,
    {
        let expr: Expr = expr.into();
        FunctionCall::new(Func::Cast).arg(expr.binary(BinOper::As, Expr::TypeName(r#type.into())))
    }

    /// Call `COALESCE` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::coalesce([
    ///         Query::select()
    ///             .from("character")
    ///             .expr(Func::max(Expr::col("size_w")))
    ///             .take()
    ///             .into(),
    ///         Expr::col("size_h"),
    ///         Expr::val(12),
    ///     ]))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT COALESCE((SELECT MAX(`size_w`) FROM `character`), `size_h`, 12) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT COALESCE((SELECT MAX("size_w") FROM "character"), "size_h", 12) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT COALESCE((SELECT MAX("size_w") FROM "character"), "size_h", 12) FROM "character""#
    /// );
    /// ```
    pub fn coalesce<I, T>(args: I) -> FunctionCall
    where
        I: IntoIterator<Item = T>,
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Coalesce).args(args.into_iter().map(Into::into))
    }

    /// Call `LOWER` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Func::lower(Expr::col("character")))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT LOWER(`character`) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT LOWER("character") FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT LOWER("character") FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("id")
    ///     .from("font")
    ///     .and_where(Func::lower(Expr::col("name")).eq("abc".trim().to_lowercase()))
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     "SELECT `id` FROM `font` WHERE LOWER(`name`) = 'abc'"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "font" WHERE LOWER("name") = 'abc'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "font" WHERE LOWER("name") = 'abc'"#
    /// );
    /// ```
    pub fn lower<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(Func::Lower).arg(expr)
    }
}
