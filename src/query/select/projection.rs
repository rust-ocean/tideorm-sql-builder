use super::*;

impl SelectStatement {
    /// Clear the select list
    pub fn clear_selects(&mut self) -> &mut Self {
        self.selects = Vec::new();
        self
    }

    /// Add an expression to the select expression list.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr(Expr::val(42))
    ///     .expr(Expr::col("id").max())
    ///     .expr((1..10_i32).fold(Expr::value(0), |expr, i| expr.add(i)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT 42, MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT 42, MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT 42, MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// ```
    pub fn expr<T>(&mut self, expr: T) -> &mut Self
    where
        T: Into<SelectExpr>,
    {
        self.selects.push(expr.into());
        self
    }

    /// Add select expressions from vector of [`SelectExpr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .exprs([
    ///         Expr::col("id").max(),
    ///         (1..10_i32).fold(Expr::value(0), |expr, i| expr.add(i)),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MAX(`id`), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("id"), 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 FROM "character""#
    /// );
    /// ```
    pub fn exprs<T, I>(&mut self, exprs: I) -> &mut Self
    where
        T: Into<SelectExpr>,
        I: IntoIterator<Item = T>,
    {
        self.selects
            .append(&mut exprs.into_iter().map(|c| c.into()).collect());
        self
    }

    pub fn exprs_mut_for_each<F>(&mut self, func: F)
    where
        F: FnMut(&mut SelectExpr),
    {
        self.selects.iter_mut().for_each(func);
    }

    /// Select distinct
    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = Some(SelectDistinct::Distinct);
        self
    }

    /// Select distinct on for *POSTGRES ONLY*: MySQL and SQLite refuse to render it.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .distinct_on(["character"])
    ///     .column("character")
    ///     .column("size_w")
    ///     .column("size_h")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT DISTINCT ON ("character") "character", "size_w", "size_h" FROM "character""#
    /// )
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .distinct_on(vec![("character", "character")])
    ///     .column("character")
    ///     .column("size_w")
    ///     .column("size_h")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT DISTINCT ON ("character"."character") "character", "size_w", "size_h" FROM "character""#
    /// )
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let distinct_cols: Vec<&str> = vec![];
    /// let query = Query::select()
    ///     .from("character")
    ///     .distinct_on(distinct_cols)
    ///     .column("character")
    ///     .column("size_w")
    ///     .column("size_h")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// )
    /// ```
    pub fn distinct_on<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        let cols = cols
            .into_iter()
            .map(|col| col.into_column_ref())
            .collect::<Vec<ColumnRef>>();
        self.distinct = if !cols.is_empty() {
            Some(SelectDistinct::DistinctOn(cols))
        } else {
            None
        };
        self
    }

    /// Add a column to the select expression list.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column("character")
    ///     .column("size_w")
    ///     .column("size_h")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column(("character", "character"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`.`character` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character"."character" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character"."character" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column(("schema", "character", "character"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `schema`.`character`.`character` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "schema"."character"."character" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "schema"."character"."character" FROM "character""#
    /// );
    /// ```
    pub fn column<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoColumnRef,
    {
        self.expr(Expr::Column(col.into_column_ref()))
    }

    /// Select columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .columns(["character", "size_w", "size_h"])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `size_w`, `size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .columns([
    ///         ("character", "character"),
    ///         ("character", "size_w"),
    ///         ("character", "size_h"),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`.`character`, `character`.`size_w`, `character`.`size_h` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character"."character", "character"."size_w", "character"."size_h" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character"."character", "character"."size_w", "character"."size_h" FROM "character""#
    /// );
    /// ```
    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.exprs(cols.into_iter().map(|c| Expr::Column(c.into_column_ref())))
    }

    /// Select column.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr_as(Expr::col("character"), "C")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" AS "C" FROM "character""#
    /// );
    /// ```
    pub fn expr_as<T, A>(&mut self, expr: T, alias: A) -> &mut Self
    where
        T: Into<Expr>,
        A: IntoIden,
    {
        self.expr(expr.alias(alias));
        self
    }

    /// Select column with window function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr_window(
    ///         Expr::col("character").max(),
    ///         WindowStatement::partition_by("font_size"),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MAX(`character`) OVER ( PARTITION BY `font_size` ) FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER ( PARTITION BY "font_size" ) FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER ( PARTITION BY "font_size" ) FROM "character""#
    /// );
    /// ```
    pub fn expr_window<T>(&mut self, expr: T, window: WindowStatement) -> &mut Self
    where
        T: Into<Expr>,
    {
        self.expr(expr.over(window));
        self
    }

    /// Select column with window function and label.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr_window_as(
    ///         Expr::col("character").max(),
    ///         WindowStatement::partition_by("font_size"),
    ///         "C",
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MAX(`character`) OVER ( PARTITION BY `font_size` ) AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER ( PARTITION BY "font_size" ) AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER ( PARTITION BY "font_size" ) AS "C" FROM "character""#
    /// );
    /// ```
    pub fn expr_window_as<T, A>(&mut self, expr: T, window: WindowStatement, alias: A) -> &mut Self
    where
        T: Into<Expr>,
        A: IntoIden,
    {
        self.expr(expr.over(window).alias(alias));
        self
    }

    /// Select column with window name.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr_window_name(Expr::col("character").max(), "w")
    ///     .window("w", WindowStatement::partition_by("font_size"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MAX(`character`) OVER `w` FROM `character` WINDOW `w` AS (PARTITION BY `font_size`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// ```
    pub fn expr_window_name<T, W>(&mut self, expr: T, window: W) -> &mut Self
    where
        T: Into<Expr>,
        W: IntoIden,
    {
        self.expr(expr.over(window));
        self
    }

    /// Select column with window name and label.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr_window_name_as(Expr::col("character").max(), "w", "C")
    ///     .window("w", WindowStatement::partition_by("font_size"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT MAX(`character`) OVER `w` AS `C` FROM `character` WINDOW `w` AS (PARTITION BY `font_size`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER "w" AS "C" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT MAX("character") OVER "w" AS "C" FROM "character" WINDOW "w" AS (PARTITION BY "font_size")"#
    /// );
    /// ```
    pub fn expr_window_name_as<T, W, A>(&mut self, expr: T, window: W, alias: A) -> &mut Self
    where
        T: Into<Expr>,
        A: IntoIden,
        W: IntoIden,
    {
        self.expr(expr.over(window).alias(alias));
        self
    }

    /// Target table for SELECT INTO clause, PostgreSQL only: MySQL and SQLite
    /// refuse to render it.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column("character")
    ///     .into_table(SelectInto::table("character_copy").modifier(SelectIntoTableModifier::Unlogged))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" INTO UNLOGGED TABLE "character_copy" FROM "character""#
    /// );
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column("character")
    ///     .into_table(
    ///         SelectInto::table("character_temp").modifier(SelectIntoTableModifier::Temporary),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" INTO TEMPORARY TABLE "character_temp" FROM "character""#
    /// );
    /// ```
    pub fn into_table(&mut self, into_table: SelectInto) -> &mut Self {
        self.into = Some(into_table);
        self
    }
}
