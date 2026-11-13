use super::*;

impl SelectStatement {
    /// From table.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("font_size")
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `font_size` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "font_size" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("font_size")
    ///     .from(("character", "glyph"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `font_size` FROM `character`.`glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "font_size" FROM "character"."glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "font_size" FROM "character"."glyph""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("font_size")
    ///     .from(("database", "character", "glyph"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `font_size` FROM `database`.`character`.`glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "font_size" FROM "database"."character"."glyph""#
    /// );
    /// // NOTE: sqlite does not support database.schema.table references
    /// ```
    ///
    /// If you specify `from` multiple times, the resulting query will have multiple from clauses.
    /// You can perform an 'old-school' join this way.
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Asterisk))
    ///     .from("character")
    ///     .from("font")
    ///     .and_where(Expr::col(("font", "id")).equals(("character", "font_id")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM `character`, `font` WHERE `font`.`id` = `character`.`font_id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    /// );
    /// ```
    pub fn from<R>(&mut self, tbl_ref: R) -> &mut Self
    where
        R: IntoTableRef,
    {
        self.from_from(tbl_ref.into_table_ref())
    }

    /// Shorthand for selecting from a constant value list.
    ///
    /// # Panics
    ///
    /// Panics on an empty values list.
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(Expr::col(Asterisk))
    ///     .from_values([(1, "hello"), (2, "world")], "x")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM (VALUES ROW(1, 'hello'), ROW(2, 'world')) AS `x`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(MariaDbDialect).unwrap(),
    ///     r#"SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS `x`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS "x""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS "x""#
    /// );
    /// ```
    pub fn from_values<I, V, A>(&mut self, value_tuples: I, alias: A) -> &mut Self
    where
        I: IntoIterator<Item = V>,
        V: IntoValueTuple,
        A: IntoIden,
    {
        let value_tuples: Vec<ValueTuple> = value_tuples
            .into_iter()
            .map(|vt| vt.into_value_tuple())
            .collect();
        assert!(!value_tuples.is_empty());
        self.from_from(TableRef::ValuesList(value_tuples, alias.into_iden()))
    }

    /// From table with alias.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from_as("character", "char")
    ///     .column(("char", "character"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `char`.`character` FROM `character` AS `char`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "char"."character" FROM "character" AS "char""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "char"."character" FROM "character" AS "char""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from_as(("font", "character"), "alias")
    ///     .column(("alias", "character"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `alias`.`character` FROM `font`.`character` AS `alias`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "alias"."character" FROM "font"."character" AS "alias""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "alias"."character" FROM "font"."character" AS "alias""#
    /// );
    /// ```
    pub fn from_as<R, A>(&mut self, tbl_ref: R, alias: A) -> &mut Self
    where
        R: IntoTableRef,
        A: IntoIden,
    {
        self.from_from(tbl_ref.into_table_ref().alias(alias.into_iden()))
    }

    /// From sub-query.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["image"])
    ///     .from_subquery(
    ///         Query::select()
    ///             .columns(["image", "aspect"])
    ///             .from("glyph")
    ///             .take(),
    ///         "subglyph",
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM (SELECT `image`, `aspect` FROM `glyph`) AS `subglyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "image" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "image" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    /// );
    /// ```
    pub fn from_subquery<T>(&mut self, query: SelectStatement, alias: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.from_from(TableRef::SubQuery(query.into(), alias.into_iden()))
    }

    /// From function call.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column(Asterisk)
    ///     .from_function(Func::random(), "func")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM RAND() AS `func`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM RANDOM() AS "func""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM RANDOM() AS "func""#
    /// );
    /// ```
    pub fn from_function<T>(&mut self, func: FunctionCall, alias: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.from_from(TableRef::FunctionCall(func, alias.into_iden()))
    }

    /// Clears all current from clauses.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column(Asterisk)
    ///     .from("character")
    ///     .from_clear()
    ///     .from("font")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM `font`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM "font""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT * FROM "font""#
    /// );
    /// ```
    pub fn from_clear(&mut self) -> &mut Self {
        self.from.clear();
        self
    }
}
