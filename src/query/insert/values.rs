use super::*;

impl InsertStatement {
    /// Construct a new [`InsertStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Take the ownership of data in the current [`SelectStatement`]
    pub fn take(&mut self) -> Self {
        Self {
            replace: self.replace,
            table: self.table.take(),
            columns: std::mem::take(&mut self.columns),
            source: self.source.take(),
            on_conflict: self.on_conflict.take(),
            returning: self.returning.take(),
            default_values: self.default_values.take(),
            with: self.with.take(),
        }
    }

    /// Use REPLACE instead of INSERT. PostgreSQL has none and refuses to render it.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .replace()
    ///     .into_table("glyph")
    ///     .columns(["aspect", "image"])
    ///     .values_panic([5.15.into(), "12A".into()])
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"REPLACE INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"REPLACE INTO "glyph" ("aspect", "image") VALUES (5.15, '12A')"#
    /// );
    /// ```
    pub fn replace(&mut self) -> &mut Self {
        self.replace = true;
        self
    }

    /// Specify which table to insert into.
    ///
    /// # Examples
    ///
    /// See [`InsertStatement::values`]
    pub fn into_table<T>(&mut self, tbl_ref: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(Box::new(tbl_ref.into_table_ref()));
        self
    }

    /// Specify what columns to insert.
    ///
    /// # Examples
    ///
    /// See [`InsertStatement::values`]
    pub fn columns<C, I>(&mut self, columns: I) -> &mut Self
    where
        C: IntoIden,
        I: IntoIterator<Item = C>,
    {
        self.columns = columns.into_iter().map(|c| c.into_iden()).collect();
        self
    }

    /// Specify a select query whose values to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["aspect", "image"])
    ///     .select_from(Query::select()
    ///         .column("aspect")
    ///         .column("image")
    ///         .from("glyph")
    ///         .and_where(Expr::col("image").like("0%"))
    ///         .take()
    ///     )
    ///     .unwrap()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) SELECT `aspect`, `image` FROM `glyph` WHERE `image` LIKE '0%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "glyph" WHERE "image" LIKE '0%'"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "glyph" WHERE "image" LIKE '0%'"#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["image"])
    ///     .select_from(
    ///         Query::select()
    ///             .expr(Expr::val("hello"))
    ///             .cond_where(Cond::all().not().add(Expr::exists(
    ///                 Query::select().expr(Expr::val("world")).take(),
    ///             )))
    ///             .take(),
    ///     )
    ///     .unwrap()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") SELECT 'hello' WHERE NOT EXISTS(SELECT 'world')"#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["image"])
    ///     .select_from(
    ///         Query::select()
    ///             .expr(Expr::col("name"))
    ///             .from("font")
    ///             .take(),
    ///     )
    ///     .unwrap()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("image") SELECT "name" FROM "font""#
    /// );
    /// ```
    pub fn select_from<S>(&mut self, select: S) -> Result<&mut Self>
    where
        S: Into<SelectStatement>,
    {
        let statement = select.into();

        if self.columns.len() != statement.selects.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: statement.selects.len(),
            });
        }

        self.source = Some(InsertValueSource::Select(Box::new(statement)));
        Ok(self)
    }

    /// Specify a row of values to be inserted.
    ///
    /// Return error when number of values not matching number of columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["aspect", "image"])
    ///     .values([
    ///         2.into(),
    ///         Func::cast_as("2020-02-02 00:00:00", "DATE").into(),
    ///     ])
    ///     .unwrap()
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2, CAST('2020-02-02 00:00:00' AS DATE))"#
    /// );
    ///
    /// assert!(
    ///     Query::insert()
    ///         .into_table("glyph")
    ///         .columns(["aspect", "image"])
    ///         .values([1.into()])
    ///         .is_err()
    /// );
    /// ```
    pub fn values<I>(&mut self, values: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = Expr>,
    {
        let values = values.into_iter().collect::<Vec<Expr>>();
        if self.columns.len() != values.len() {
            return Err(Error::ColValNumMismatch {
                col_len: self.columns.len(),
                val_len: values.len(),
            });
        }
        if !values.is_empty() {
            let values_source = if let Some(InsertValueSource::Values(values)) = &mut self.source {
                values
            } else {
                self.source = Some(InsertValueSource::Values(Default::default()));
                if let Some(InsertValueSource::Values(values)) = &mut self.source {
                    values
                } else {
                    unreachable!();
                }
            };
            values_source.push(values);
        }
        Ok(self)
    }

    /// Specify a row of values to be inserted, variation of [`InsertStatement::values`].
    ///
    /// # Panics
    ///
    /// Panics when number of values not matching number of columns.
    /// The engine's `insert_many` does not panic: it builds the column list from the active models.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["aspect", "image"])
    ///     .values_panic([2.1345.into(), "24B".into()])
    ///     .values_panic([5.15.into(), "12A".into()])
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// ```
    ///
    /// ```should_panic
    /// # use sql_builder::*;
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["aspect"])
    ///     .values_panic([2.1345.into(), "24B".into()])
    ///     .take();
    /// ```
    pub fn values_panic<I>(&mut self, values: I) -> &mut Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.values(values).unwrap()
    }

    /// Add rows to be inserted from an iterator, variation of [`InsertStatement::values_panic`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let rows = vec![[2.1345.into(), "24B".into()], [5.15.into(), "12A".into()]];
    ///
    /// let query = Query::insert()
    ///     .into_table("glyph")
    ///     .columns(["aspect", "image"])
    ///     .values_from_panic(rows)
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"INSERT INTO "glyph" ("aspect", "image") VALUES (2.1345, '24B'), (5.15, '12A')"#
    /// );
    /// ```
    pub fn values_from_panic<I, J>(&mut self, values_iter: J) -> &mut Self
    where
        I: IntoIterator<Item = Expr>,
        J: IntoIterator<Item = I>,
    {
        values_iter.into_iter().for_each(|values| {
            self.values_panic(values);
        });
        self
    }
}
