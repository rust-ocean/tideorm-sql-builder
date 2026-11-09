use super::*;

impl Expr {
    /// Express a `EXISTS` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(Expr::exists(Query::select().column("id").from("character").take()), "character_exists")
    ///     .expr_as(Expr::exists(Query::select().column("id").from("glyph").take()), "glyph_exists")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT EXISTS(SELECT `id` FROM `character`) AS `character_exists`, EXISTS(SELECT `id` FROM `glyph`) AS `glyph_exists`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT EXISTS(SELECT "id" FROM "character") AS "character_exists", EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// ```
    pub fn exists(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Exists), Box::new(sel.into()))
    }

    /// Express a `NOT EXISTS` sub-query expression.
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(Expr::not_exists(Query::select().column("id").from("character").take()), "character_exists")
    ///     .expr_as(Expr::not_exists(Query::select().column("id").from("glyph").take()), "glyph_exists")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT NOT EXISTS(SELECT `id` FROM `character`) AS `character_exists`, NOT EXISTS(SELECT `id` FROM `glyph`) AS `glyph_exists`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT NOT EXISTS(SELECT "id" FROM "character") AS "character_exists", NOT EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT NOT EXISTS(SELECT "id" FROM "character") AS "character_exists", NOT EXISTS(SELECT "id" FROM "glyph") AS "glyph_exists""#
    /// );
    /// ```
    pub fn not_exists(sel: SelectStatement) -> Self {
        Self::exists(sel).not()
    }

    /// Express a `ANY` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("id")
    ///     .from("character")
    ///     .and_where(Expr::col("id").eq(Expr::any(
    ///         Query::select().column("id").from("character").take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `id` FROM `character` WHERE `id` = ANY(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "character" WHERE "id" = ANY(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn any(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Any), Box::new(sel.into()))
    }

    /// Express a `SOME` sub-query expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("id")
    ///     .from("character")
    ///     .and_where(Expr::col("id").ne(Expr::some(
    ///         Query::select().column("id").from("character").take(),
    ///     )))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `id` FROM `character` WHERE `id` <> SOME(SELECT `id` FROM `character`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "character" WHERE "id" <> SOME(SELECT "id" FROM "character")"#
    /// );
    /// ```
    pub fn some(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::Some), Box::new(sel.into()))
    }

    /// Express a `ALL` sub-query expression.
    pub fn all(sel: SelectStatement) -> Self {
        Self::SubQuery(Some(SubQueryOper::All), Box::new(sel.into()))
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///                 Expr::col(("glyph", "aspect")).is_in([2, 4]),
    ///                 true
    ///              )
    ///             .finally(false),
    ///          "is_even"
    ///     )
    ///     .from("glyph")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );
    /// ```
    pub fn case<C, T>(cond: C, then: T) -> CaseStatement
    where
        C: IntoCondition,
        T: Into<Self>,
    {
        CaseStatement::new().case(cond, then)
    }

    /// Keyword `NULL`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(Expr::null()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT NULL"#);
    /// assert_eq!(query.to_string(PostgresQueryBuilder).unwrap(), r#"SELECT NULL"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder).unwrap(), r#"SELECT NULL"#);
    /// ```
    pub fn null() -> Self {
        Self::Keyword(Keyword::Null)
    }

    /// Keyword `CURRENT_DATE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(Expr::current_date()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT CURRENT_DATE"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_DATE"#
    /// );
    /// ```
    pub fn current_date() -> Self {
        Self::Keyword(Keyword::CurrentDate)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(Expr::current_time()).to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT CURRENT_TIME"#);
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_TIME"#
    /// );
    /// ```
    pub fn current_time() -> Self {
        Self::Keyword(Keyword::CurrentTime)
    }

    /// Keyword `CURRENT_TIMESTAMP`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder};
    ///
    /// let query = Query::select().expr(Expr::current_timestamp()).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT CURRENT_TIMESTAMP"#
    /// );
    /// ```
    pub fn current_timestamp() -> Self {
        Self::Keyword(Keyword::CurrentTimestamp)
    }

    /// Keyword `DEFAULT`.
    ///
    /// SQLite has no `DEFAULT` value and refuses to render it.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{
    ///     Expr, MysqlQueryBuilder, PostgresQueryBuilder, Query, SqliteQueryBuilder,
    /// };
    ///
    /// let query = Query::insert()
    ///     .columns(["id"])
    ///     .values_panic([Expr::keyword_default()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT (`id`) VALUES (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT ("id") VALUES (DEFAULT)"#
    /// );
    /// assert!(query.to_string(SqliteQueryBuilder).is_err());
    /// ```
    pub fn keyword_default() -> Self {
        Self::Keyword(Keyword::Default)
    }

    /// Custom keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::custom_keyword("test"))
    ///     .to_owned();
    ///
    /// assert_eq!(query.to_string(MysqlQueryBuilder).unwrap(), r#"SELECT test"#);
    /// assert_eq!(query.to_string(PostgresQueryBuilder).unwrap(), r#"SELECT test"#);
    /// assert_eq!(query.to_string(SqliteQueryBuilder).unwrap(), r#"SELECT test"#);
    /// ```
    pub fn custom_keyword<T>(i: T) -> Self
    where
        T: IntoIden,
    {
        Self::Keyword(Keyword::Custom(i.into_iden()))
    }

    pub(crate) fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_, _, _))
    }

    pub(crate) fn get_bin_oper(&self) -> Option<&BinOper> {
        match self {
            Self::Binary(_, oper, _) => Some(oper),
            _ => None,
        }
    }
}
