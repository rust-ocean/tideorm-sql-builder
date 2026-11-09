use super::*;

impl SelectStatement {
    /// Group by columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .column(("font", "name"))
    ///     .from("character")
    ///     .join(JoinType::RightJoin, "font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .group_by_columns([
    ///         "character",
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character""#
    /// );
    /// ```
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .column(("font", "name"))
    ///     .from("character")
    ///     .join(JoinType::RightJoin, "font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .group_by_columns([
    ///         ("character", "character"),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`.`character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// ```
    pub fn group_by_columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        self.add_group_by(cols.into_iter().map(|c| Expr::Column(c.into_column_ref())))
    }

    /// Add a group by column.
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .column(("font", "name"))
    ///     .from("character")
    ///     .join(JoinType::RightJoin, "font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .group_by_col(("character", "character"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` GROUP BY `character`.`character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" GROUP BY "character"."character""#
    /// );
    /// ```
    pub fn group_by_col<T>(&mut self, col: T) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.group_by_columns([col])
    }

    /// Clear the group by expressions.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column("character")
    ///     .add_group_by([Expr::col("size_w").into()])
    ///     .clear_group_by()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character""#
    /// );
    /// ```
    pub fn clear_group_by(&mut self) -> &mut Self {
        self.groups = Vec::new();
        self
    }

    /// Add group by expressions from vector of [`SelectExpr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .column("character")
    ///     .add_group_by([Expr::col("size_w").into(), Expr::col("size_h").into()])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` GROUP BY `size_w`, `size_h`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" GROUP BY "size_w", "size_h""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" GROUP BY "size_w", "size_h""#
    /// );
    /// ```
    pub fn add_group_by<I>(&mut self, expr: I) -> &mut Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.groups.append(&mut expr.into_iter().collect());
        self
    }

    /// Having condition, expressed with [`any!`](crate::any) and [`all!`](crate::all).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("aspect")
    ///     .expr(Expr::col("image").max())
    ///     .from("glyph")
    ///     .group_by_columns([
    ///         "aspect",
    ///     ])
    ///     .cond_having(
    ///         all![
    ///             Expr::col(("glyph", "aspect")).is_in([3, 4]),
    ///             any![
    ///                 Expr::col(("glyph", "image")).like("A%"),
    ///                 Expr::col(("glyph", "image")).like("B%")
    ///             ]
    ///         ]
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `glyph`.`aspect` IN (3, 4) AND (`glyph`.`image` LIKE 'A%' OR `glyph`.`image` LIKE 'B%')"#
    /// );
    /// ```
    pub fn cond_having<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition,
    {
        self.having.add_condition(condition.into_condition());
        self
    }

    /// Clear the having condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("glyph")
    ///     .column("aspect")
    ///     .group_by_columns(["aspect"])
    ///     .cond_having(Expr::col("aspect").gt(2))
    ///     .clear_having()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect` FROM `glyph` GROUP BY `aspect`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" GROUP BY "aspect""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" GROUP BY "aspect""#
    /// );
    /// ```
    pub fn clear_having(&mut self) -> &mut Self {
        self.having = ConditionHolder::new();
        self
    }

    /// And having condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("aspect")
    ///     .expr(Expr::col("image").max())
    ///     .from("glyph")
    ///     .group_by_columns([
    ///         "aspect",
    ///     ])
    ///     .and_having(Expr::col("aspect").gt(2))
    ///     .cond_having(Expr::col("aspect").lt(8))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect`, MAX(`image`) FROM `glyph` GROUP BY `aspect` HAVING `aspect` > 2 AND `aspect` < 8"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2 AND "aspect" < 8"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2 AND "aspect" < 8"#
    /// );
    /// ```
    pub fn and_having(&mut self, other: Expr) -> &mut Self {
        self.cond_having(other)
    }
}
