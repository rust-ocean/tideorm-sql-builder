use super::*;

impl SelectStatement {
    /// Join with other table by [`JoinType`].
    ///
    /// If [`JoinType`] is [`CrossJoin`](JoinType::CrossJoin), the condition will be ignored.
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
    ///         .join(
    ///             JoinType::RightJoin,
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
    pub fn join<R, C>(&mut self, join: JoinType, tbl_ref: R, condition: C) -> &mut Self
    where
        R: IntoTableRef,
        C: IntoCondition,
    {
        let on = match join {
            JoinType::CrossJoin => None,
            _ => Some(JoinOn::Condition(Box::new(
                ConditionHolder::new_with_condition(condition.into_condition()),
            ))),
        };
        self.push_join(join, tbl_ref.into_table_ref(), on, false)
    }

    /// Join with other table by [`JoinType`], assigning an alias to the joined table.
    ///
    /// If [`JoinType`] is [`CrossJoin`](JoinType::CrossJoin), the condition will be ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .column(("f", "name"))
    ///     .from("character")
    ///     .join_as(
    ///         JoinType::RightJoin,
    ///         "font",
    ///         "f",
    ///         Expr::col(("character", "font_id")).equals(("f", "id"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `f`.`name` FROM `character` RIGHT JOIN `font` AS `f` ON `character`.`font_id` = `f`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "f"."name" FROM "character" RIGHT JOIN "font" AS "f" ON "character"."font_id" = "f"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "f"."name" FROM "character" RIGHT JOIN "font" AS "f" ON "character"."font_id" = "f"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// assert_eq!(
    ///     Query::select()
    ///         .column("character")
    ///         .column(("f", "name"))
    ///         .from("character")
    ///         .join_as(
    ///             JoinType::RightJoin,
    ///             "font",
    ///             "f",
    ///             Condition::all()
    ///                 .add(Expr::col(("character", "font_id")).equals(("f", "id")))
    ///                 .add(Expr::col(("character", "font_id")).equals(("f", "id")))
    ///         )
    ///         .to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character`, `f`.`name` FROM `character` RIGHT JOIN `font` AS `f` ON `character`.`font_id` = `f`.`id` AND `character`.`font_id` = `f`.`id`"#
    /// );
    /// ```
    pub fn join_as<R, A, C>(
        &mut self,
        join: JoinType,
        tbl_ref: R,
        alias: A,
        condition: C,
    ) -> &mut Self
    where
        R: IntoTableRef,
        A: IntoIden,
        C: IntoCondition,
    {
        let on = match join {
            JoinType::CrossJoin => None,
            _ => Some(JoinOn::Condition(Box::new(
                ConditionHolder::new_with_condition(condition.into_condition()),
            ))),
        };
        self.push_join(
            join,
            tbl_ref.into_table_ref().alias(alias.into_iden()),
            on,
            false,
        )
    }

    /// Join with sub-query.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("name")
    ///     .from("font")
    ///     .join_subquery(
    ///         JoinType::LeftJoin,
    ///         Query::select().column("id").from("glyph").take(),
    ///         "sub_glyph",
    ///         Expr::col(("font", "id")).equals(("sub_glyph", "id"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// assert_eq!(
    ///     Query::select()
    ///         .column("name")
    ///         .from("font")
    ///         .join_subquery(
    ///             JoinType::LeftJoin,
    ///             Query::select().column("id").from("glyph").take(),
    ///             "sub_glyph",
    ///             Condition::all()
    ///                 .add(Expr::col(("font", "id")).equals(("sub_glyph", "id")))
    ///                 .add(Expr::col(("font", "id")).equals(("sub_glyph", "id")))
    ///         )
    ///         .to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id` AND `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// ```
    pub fn join_subquery<T, C>(
        &mut self,
        join: JoinType,
        query: SelectStatement,
        alias: T,
        condition: C,
    ) -> &mut Self
    where
        T: IntoIden,
        C: IntoCondition,
    {
        self.push_join(
            join,
            TableRef::SubQuery(query.into(), alias.into_iden()),
            JoinOn::Condition(Box::new(ConditionHolder::new_with_condition(
                condition.into_condition(),
            ))),
            false,
        )
    }

    /// Join Lateral with sub-query. Not supported by SQLite.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("name")
    ///     .from("font")
    ///     .join_lateral(
    ///         JoinType::LeftJoin,
    ///         Query::select().column("id").from("glyph").take(),
    ///         "sub_glyph",
    ///         Expr::col(("font", "id")).equals(("sub_glyph", "id"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN LATERAL (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "name" FROM "font" LEFT JOIN LATERAL (SELECT "id" FROM "glyph") AS "sub_glyph" ON "font"."id" = "sub_glyph"."id""#
    /// );
    ///
    /// // Constructing chained join conditions
    /// assert_eq!(
    ///     Query::select()
    ///         .column("name")
    ///         .from("font")
    ///         .join_lateral(
    ///             JoinType::LeftJoin,
    ///             Query::select().column("id").from("glyph").take(),
    ///             "sub_glyph",
    ///             Condition::all()
    ///                 .add(Expr::col(("font", "id")).equals(("sub_glyph", "id")))
    ///                 .add(Expr::col(("font", "id")).equals(("sub_glyph", "id")))
    ///         )
    ///         .to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `name` FROM `font` LEFT JOIN LATERAL (SELECT `id` FROM `glyph`) AS `sub_glyph` ON `font`.`id` = `sub_glyph`.`id` AND `font`.`id` = `sub_glyph`.`id`"#
    /// );
    /// ```
    pub fn join_lateral<T, C>(
        &mut self,
        join: JoinType,
        query: SelectStatement,
        alias: T,
        condition: C,
    ) -> &mut Self
    where
        T: IntoIden,
        C: IntoCondition,
    {
        self.push_join(
            join,
            TableRef::SubQuery(query.into(), alias.into_iden()),
            JoinOn::Condition(Box::new(ConditionHolder::new_with_condition(
                condition.into_condition(),
            ))),
            true,
        )
    }

    pub(super) fn push_join(
        &mut self,
        join: JoinType,
        table: TableRef,
        on: impl Into<Option<JoinOn>>,
        lateral: bool,
    ) -> &mut Self {
        self.join.push(JoinExpr {
            join,
            table: Box::new(table),
            on: on.into(),
            lateral,
        });
        self
    }
}
