use super::*;

impl Expr {
    /// Express the target column without table prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::col("size_w").eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::col(("character", "size_w")).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn col<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::Column(n.into_column_ref())
    }

    /// Express the target column without table prefix, returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::column("size_w").eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 1"#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::column(("character", "size_w")).eq(1))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `character`.`size_w` = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."size_w" = 1"#
    /// );
    /// ```
    pub fn column<T>(n: T) -> Self
    where
        T: IntoColumnRef,
    {
        Self::Column(n.into_column_ref())
    }

    /// Wraps tuple of `Expr`, can be used for tuple comparison
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(
    ///         Expr::tuple([Expr::col("size_w").into(), Expr::value(100)])
    ///             .lt(Expr::tuple([Expr::value(500), Expr::value(100)])))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE (`size_w`, 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w", 100) < (500, 100)"#
    /// );
    /// ```
    pub fn tuple<I>(n: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self::Tuple(n.into_iter().collect::<Vec<Self>>())
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::val(1).into())
    ///     .and_where(Expr::val(2.5).into())
    ///     .and_where(Expr::val("3").into())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn val<V>(v: V) -> Self
    where
        V: Into<Value>,
    {
        Self::from(v)
    }

    /// Wrap an expression to perform some operation on it later.
    ///
    /// This is rarely necessary: the SQL operations are also [`ExprTrait`] methods,
    /// which can be called directly on any expression type.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     // Wrapped in `Expr::expr`:
    ///     .and_where(Expr::expr(Expr::col("size_w").if_null(0)).gt(2))
    ///     .to_owned();
    ///
    /// // The same expression without the wrapper:
    /// let _ = Expr::col("size_w").if_null(0).gt(2);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE IFNULL(`size_w`, 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE COALESCE("size_w", 0) > 2"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE IFNULL("size_w", 0) > 2"#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     // Wrapped in `Expr::expr`:
    ///     .and_where(Expr::expr(Func::lower(Expr::col("character"))).is_in(["a", "b"]))
    ///     .to_owned();
    ///
    /// // The same expression without the wrapper:
    /// let _ = Func::lower(Expr::col("character")).is_in(["a", "b"]);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE LOWER(`character`) IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE LOWER("character") IN ('a', 'b')"#
    /// );
    /// ```
    #[allow(clippy::self_named_constructors)]
    pub fn expr<T>(expr: T) -> Self
    where
        T: Into<Self>,
    {
        expr.into()
    }

    /// Express a [`Value`], returning a [`Expr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::value(1))
    ///     .and_where(Expr::value(2.5))
    ///     .and_where(Expr::value("3"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 AND 2.5 AND '3'"#
    /// );
    /// ```
    pub fn value<V>(v: V) -> Self
    where
        V: Into<Self>,
    {
        v.into()
    }

    /// Express any custom expression in [`&str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::cust("1 = 1"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE 1 = 1"#
    /// );
    /// ```
    pub fn cust<T>(s: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::Custom(s.into())
    }

    /// Express any custom expression with [`Value`]. Use this if your expression needs variables.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::col("id").eq(1))
    ///     .and_where(Expr::cust_with_values("6 = ? * ?", [2, 3], &MysqlQueryBuilder).unwrap())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character` WHERE `id` = 1 AND (6 = 2 * 3)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::col("id").eq(1))
    ///     .and_where(Expr::cust_with_values("6 = $2 * $1", [3, 2], &PostgresQueryBuilder).unwrap())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND (6 = 2 * 3)"#
    /// );
    /// ```
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("6 = ? * ?", [2, 3], &MysqlQueryBuilder).unwrap())
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT 6 = 2 * 3"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder).unwrap(), r#"SELECT 6 = 2 * 3"#);
    /// ```
    /// A marker inside a string, a quoted name or a comment is text, read by
    /// the builder's own rules: here a PostgreSQL dollar-quoted string.
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("$1 || $$ $2 $$", ["a"], &PostgresQueryBuilder).unwrap())
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(PostgresQueryBuilder).unwrap(), r#"SELECT 'a' || $$ $2 $$"#);
    /// ```
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_values("data @? ($1::JSONPATH)", ["hello"], &PostgresQueryBuilder).unwrap())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT data @? ('hello'::JSONPATH)"#
    /// );
    /// ```
    pub fn cust_with_values<T, V, I>(
        s: T,
        v: I,
        builder: &impl QueryBuilder,
    ) -> Result<Self, crate::error::RenderError>
    where
        T: Into<Cow<'static, str>>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        Self::cust_with_exprs(
            s,
            v.into_iter().map(|v| Into::<Value>::into(v).into()),
            builder,
        )
    }

    /// Express any custom expression with [`Expr`]. Use this if your expression needs other expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::val(1).add(2))
    ///     .expr(Expr::cust_with_expr("data @? ($1::JSONPATH)", "hello", &PostgresQueryBuilder).unwrap())
    ///     .to_owned();
    /// let (sql, values) = query.build(PostgresQueryBuilder).unwrap();
    ///
    /// assert_eq!(sql, r#"SELECT $1 + $2, data @? ($3::JSONPATH)"#);
    /// assert_eq!(
    ///     values,
    ///     Values(vec![1i32.into(), 2i32.into(), "hello".into()])
    /// );
    /// ```
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::cust_with_expr(
    ///         "json_agg(DISTINCT $1)",
    ///         Expr::col("character"),
    ///         &PostgresQueryBuilder,
    ///     ).unwrap())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT json_agg(DISTINCT "character")"#
    /// );
    /// ```
    pub fn cust_with_expr<T, E>(
        s: T,
        expr: E,
        builder: &impl QueryBuilder,
    ) -> Result<Self, crate::error::RenderError>
    where
        T: Into<Cow<'static, str>>,
        E: Into<Self>,
    {
        Self::cust_with_exprs(s, [expr.into()], builder)
    }

    /// Express any custom expression with [`Expr`]. Use this if your expression needs other expressions.
    pub fn cust_with_exprs<T, I>(
        s: T,
        v: I,
        builder: &impl QueryBuilder,
    ) -> Result<Self, crate::error::RenderError>
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = Expr>,
    {
        CustomExpr::new(&s.into(), v.into_iter().collect(), builder).map(Self::CustomWithExpr)
    }
}
