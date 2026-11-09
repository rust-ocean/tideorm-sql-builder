use super::*;

impl InsertStatement {
    /// ON CONFLICT expression
    ///
    /// # Examples
    ///
    /// - [`OnConflict::update_columns`]: Update column value of existing row with inserting value
    pub fn on_conflict(&mut self, on_conflict: OnConflict) -> &mut Self {
        self.on_conflict = Some(on_conflict);
        self
    }

    /// Whether the statement says what to do on a conflict.
    pub fn has_on_conflict(&self) -> bool {
        self.on_conflict.is_some()
    }

    /// RETURNING expressions.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["image"])
    ///     .values_panic(["12A".into()])
    ///     .returning(Query::returning().columns(["id"]))
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A') RETURNING `id`"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// ```
    pub fn returning(&mut self, returning: ReturningClause) -> &mut Self {
        self.returning = Some(returning);
        self
    }

    /// RETURNING expressions for a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["image"])
    ///     .values_panic(["12A".into()])
    ///     .returning_col("id")
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A') RETURNING `id`"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING "id""#
    /// );
    /// ```
    pub fn returning_col<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoColumnRef,
    {
        self.returning(ReturningClause::Columns(vec![col.into_column_ref()]))
    }

    /// RETURNING expressions all columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["image"])
    ///     .values_panic(["12A".into()])
    ///     .returning_all()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     "INSERT INTO `glyph` (`image`) VALUES ('12A') RETURNING *"
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING *"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('12A') RETURNING *"#
    /// );
    /// ```
    pub fn returning_all(&mut self) -> &mut Self {
        self.returning(ReturningClause::All)
    }

    /// Create a [WithQuery] by specifying a [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, IntoCondition, IntoIden};
    ///
    /// let select = SelectStatement::new()
    ///         .columns(["id", "image", "aspect"])
    ///         .from("glyph")
    ///         .take();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column("id")
    ///         .column("image")
    ///         .column("aspect")
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let select = SelectStatement::new()
    ///         .columns(["id", "image", "aspect"])
    ///         .from("cte")
    ///         .take();
    ///     let mut insert = Query::insert();
    ///     insert
    ///         .into_table("glyph")
    ///         .columns(["id", "image", "aspect"])
    ///         .select_from(select)
    ///         .unwrap();
    ///     let query = insert.with(with_clause);
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH `cte` (`id`, `image`, `aspect`) AS (SELECT `id`, `image`, `aspect` FROM `glyph`) INSERT INTO `glyph` (`id`, `image`, `aspect`) SELECT `id`, `image`, `aspect` FROM `cte`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph") INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph") INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#
    /// );
    /// ```
    pub fn with(self, clause: WithClause) -> WithQuery {
        clause.query(self)
    }

    /// Create a Common Table Expression by specifying a [CommonTableExpression][crate::CommonTableExpression]
    /// or [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, IntoCondition, IntoIden};
    ///
    /// let select = SelectStatement::new()
    ///         .columns(["id", "image", "aspect"])
    ///         .from("glyph")
    ///         .take();
    ///     let cte = CommonTableExpression::new()
    ///         .query(select)
    ///         .column("id")
    ///         .column("image")
    ///         .column("aspect")
    ///         .table_name("cte")
    ///         .to_owned();
    ///     let with_clause = WithClause::new().cte(cte).to_owned();
    ///     let select = SelectStatement::new()
    ///         .columns(["id", "image", "aspect"])
    ///         .from("cte")
    ///         .take();
    ///     let mut query = Query::insert();
    ///     query
    ///         .with_cte(with_clause)
    ///         .into_table("glyph")
    ///         .columns(["id", "image", "aspect"])
    ///         .select_from(select)
    ///         .unwrap();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH `cte` (`id`, `image`, `aspect`) AS (SELECT `id`, `image`, `aspect` FROM `glyph`) INSERT INTO `glyph` (`id`, `image`, `aspect`) SELECT `id`, `image`, `aspect` FROM `cte`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph") INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph") INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#
    /// );
    /// ```
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(clause.into());
        self
    }

    /// Insert with default values if columns and values are not supplied.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// // Insert default
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .or_default_values()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` VALUES ()"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" VALUES (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" DEFAULT VALUES"#
    /// );
    ///
    /// // Ordinary insert as columns and values are supplied
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .or_default_values()
    ///     .columns(["image"])
    ///     .values_panic(["ABC".into()])
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` (`image`) VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// ```
    pub fn or_default_values(&mut self) -> &mut Self {
        self.default_values = Some(1);
        self
    }

    /// Insert multiple rows with default values if columns and values are not supplied.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// // Insert default
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .or_default_values_many(3)
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` VALUES (), (), ()"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" VALUES (DEFAULT), (DEFAULT), (DEFAULT)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" (rowid) VALUES (NULL), (NULL), (NULL)"#
    /// );
    ///
    /// // Ordinary insert as columns and values are supplied
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .or_default_values_many(3)
    ///     .columns(["image"])
    ///     .values_panic(["ABC".into()])
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` (`image`) VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") VALUES ('ABC')"#
    /// );
    /// ```
    pub fn or_default_values_many(&mut self, num_rows: usize) -> &mut Self {
        self.default_values = Some(num_rows);
        self
    }
}
