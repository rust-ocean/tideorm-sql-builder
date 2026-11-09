use super::*;

impl SelectStatement {
    #[allow(clippy::wrong_self_convention)]
    pub(super) fn from_from(&mut self, select: TableRef) -> &mut Self {
        self.from.push(select);
        self
    }

    /// Cross join.
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
    ///     .cross_join("font")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` CROSS JOIN `font`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" CROSS JOIN "font""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" CROSS JOIN "font""#
    /// );
    /// ```
    pub fn cross_join<R>(&mut self, tbl_ref: R) -> &mut Self
    where
        R: IntoTableRef,
    {
        self.push_join(JoinType::CrossJoin, tbl_ref.into_table_ref(), None, false);
        self
    }

    /// Left join.
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
    ///     .left_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column("character")
    ///         .column(("font", "name"))
    ///         .from("character")
    ///         .left_join(
    ///             "font",
    ///             all![
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn left_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::LeftJoin, tbl_ref, condition)
    }

    /// Right join.
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
    ///     .right_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column("character")
    ///         .column(("font", "name"))
    ///         .from("character")
    ///         .right_join(
    ///             "font",
    ///             all![
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` RIGHT JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" RIGHT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn right_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::RightJoin, tbl_ref, condition)
    }

    /// Inner join.
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
    ///     .inner_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` INNER JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column("character")
    ///         .column(("font", "name"))
    ///         .from("character")
    ///         .inner_join(
    ///             "font",
    ///             all![
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` INNER JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn inner_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::InnerJoin, tbl_ref, condition)
    }

    /// Full outer join.
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
    ///     .full_outer_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column("character")
    ///         .column(("font", "name"))
    ///         .from("character")
    ///         .full_outer_join(
    ///             "font",
    ///             all![
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "font"."name" FROM "character" FULL OUTER JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    /// );
    /// ```
    pub fn full_outer_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::FullOuterJoin, tbl_ref, condition)
    }

    /// Straight join. MySQL only.
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
    ///     .straight_join("font", Expr::col(("character", "font_id")).equals(("font", "id")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` STRAIGHT_JOIN `font` ON `character`.`font_id` = `font`.`id`"#
    /// );
    ///
    /// // Constructing chained join conditions
    /// let query = Query::select()
    ///         .column("character")
    ///         .column(("font", "name"))
    ///         .from("character")
    ///         .straight_join(
    ///             "font",
    ///             all![
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///                 Expr::col(("character", "font_id")).equals(("font", "id")),
    ///             ]
    ///         )
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `font`.`name` FROM `character` STRAIGHT_JOIN `font` ON `character`.`font_id` = `font`.`id` AND `character`.`font_id` = `font`.`id`"#
    /// );
    /// ```
    pub fn straight_join<R, C>(&mut self, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        self.join(JoinType::StraightJoin, tbl_ref, condition)
    }
}
