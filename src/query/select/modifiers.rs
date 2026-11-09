use super::*;

impl SelectStatement {
    /// Limit the number of returned rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("aspect")
    ///     .from("glyph")
    ///     .limit(10)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10"#
    /// );
    /// ```
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit.into());
        self
    }

    /// Reset limit
    pub fn reset_limit(&mut self) -> &mut Self {
        self.limit = None;
        self
    }

    /// Offset number of returned rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("aspect")
    ///     .from("glyph")
    ///     .limit(10)
    ///     .offset(10)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect` FROM `glyph` LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" LIMIT 10 OFFSET 10"#
    /// );
    /// ```
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset.into());
        self
    }

    /// Reset offset
    pub fn reset_offset(&mut self) -> &mut Self {
        self.offset = None;
        self
    }

    /// Row locking (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .lock(LockType::Update)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn lock(&mut self, r#type: LockType) -> &mut Self {
        self.lock = Some(LockClause {
            r#type,
            tables: Vec::new(),
            behavior: None,
        });
        self
    }

    /// Row locking with tables (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .lock_with_tables(LockType::Update, ["glyph"])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE OF `glyph`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE OF "glyph""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn lock_with_tables<T, I>(&mut self, r#type: LockType, tables: I) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIterator<Item = T>,
    {
        self.lock = Some(LockClause {
            r#type,
            tables: tables.into_iter().map(|t| t.into_table_ref()).collect(),
            behavior: None,
        });
        self
    }

    /// Row locking with behavior (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .lock_with_behavior(LockType::Update, LockBehavior::Nowait)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn lock_with_behavior(&mut self, r#type: LockType, behavior: LockBehavior) -> &mut Self {
        self.lock = Some(LockClause {
            r#type,
            tables: Vec::new(),
            behavior: Some(behavior),
        });
        self
    }

    /// Row locking with tables and behavior (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .lock_with_tables_behavior(LockType::Update, ["glyph"], LockBehavior::Nowait)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE OF `glyph` NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE OF "glyph" NOWAIT"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn lock_with_tables_behavior<T, I>(
        &mut self,
        r#type: LockType,
        tables: I,
        behavior: LockBehavior,
    ) -> &mut Self
    where
        T: IntoTableRef,
        I: IntoIterator<Item = T>,
    {
        self.lock = Some(LockClause {
            r#type,
            tables: tables.into_iter().map(|t| t.into_table_ref()).collect(),
            behavior: Some(behavior),
        });
        self
    }

    /// Shared row locking (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .lock_shared()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 LOCK IN SHARE MODE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR SHARE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn lock_shared(&mut self) -> &mut Self {
        self.lock(LockType::Share)
    }

    /// Exclusive row locking (if supported).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .lock_exclusive()
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 FOR UPDATE"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn lock_exclusive(&mut self) -> &mut Self {
        self.lock(LockType::Update)
    }

    /// Union with another SelectStatement that must have the same selected fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .union(UnionType::All, Query::select()
    ///         .column("character")
    ///         .from("character")
    ///         .and_where(Expr::col("font_id").eq(4))
    ///         .to_owned()
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 UNION ALL (SELECT `character` FROM `character` WHERE `font_id` = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL (SELECT "character" FROM "character" WHERE "font_id" = 4)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL SELECT "character" FROM "character" WHERE "font_id" = 4"#
    /// );
    /// ```
    pub fn union(&mut self, union_type: UnionType, query: SelectStatement) -> &mut Self {
        self.unions.push((union_type, query));
        self
    }

    /// Union with multiple SelectStatement that must have the same selected fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .and_where(Expr::col("font_id").eq(5))
    ///     .unions([
    ///         (UnionType::All, Query::select()
    ///             .column("character")
    ///             .from("character")
    ///             .and_where(Expr::col("font_id").eq(4))
    ///             .to_owned()),
    ///         (UnionType::Distinct, Query::select()
    ///             .column("image")
    ///             .from("glyph")
    ///             .to_owned()),
    ///     ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5 UNION ALL (SELECT `character` FROM `character` WHERE `font_id` = 4) UNION (SELECT `image` FROM `glyph`)"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL (SELECT "character" FROM "character" WHERE "font_id" = 4) UNION (SELECT "image" FROM "glyph")"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5 UNION ALL SELECT "character" FROM "character" WHERE "font_id" = 4 UNION SELECT "image" FROM "glyph""#
    /// );
    /// ```
    pub fn unions<T: IntoIterator<Item = (UnionType, SelectStatement)>>(
        &mut self,
        unions: T,
    ) -> &mut Self {
        self.unions.extend(unions);
        self
    }

    /// Create a [WithQuery] by specifying a [WithClause] to execute this query with.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, IntoCondition, IntoIden};
    ///
    /// let base_query = SelectStatement::new()
    ///                     .column("id")
    ///                     .expr(1i32)
    ///                     .column("next")
    ///                     .column("value")
    ///                     .from("task")
    ///                     .to_owned();
    ///
    /// let cte_referencing = SelectStatement::new()
    ///                             .column("id")
    ///                             .expr(Expr::col("depth").add(1i32))
    ///                             .column("next")
    ///                             .column("value")
    ///                             .from("task")
    ///                             .join(
    ///                                 JoinType::InnerJoin,
    ///                                 "cte_traversal",
    ///                                 Expr::col(("cte_traversal", "next")).equals(("task", "id"))
    ///                             )
    ///                             .to_owned();
    ///
    /// let common_table_expression = CommonTableExpression::new()
    ///             .query(
    ///                 base_query.clone().union(UnionType::All, cte_referencing).to_owned()
    ///             )
    ///             .columns(["id", "depth", "next", "value"])
    ///             .table_name("cte_traversal")
    ///             .to_owned();
    ///
    /// let select = SelectStatement::new()
    ///         .column(Asterisk)
    ///         .from("cte_traversal")
    ///         .to_owned();
    ///
    /// let with_clause = WithClause::new()
    ///         .recursive(true)
    ///         .cte(common_table_expression)
    ///         .to_owned();
    ///
    /// let query = select.with(with_clause).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, 1, `next`, `value` FROM `task` UNION ALL (SELECT `id`, `depth` + 1, `next`, `value` FROM `task` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `task`.`id`)) SELECT * FROM `cte_traversal`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "task" UNION ALL (SELECT "id", "depth" + 1, "next", "value" FROM "task" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "task"."id")) SELECT * FROM "cte_traversal""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "task" UNION ALL SELECT "id", "depth" + 1, "next", "value" FROM "task" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "task"."id") SELECT * FROM "cte_traversal""#
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
    /// let base_query = SelectStatement::new()
    ///                     .column("id")
    ///                     .expr(1i32)
    ///                     .column("next")
    ///                     .column("value")
    ///                     .from("task")
    ///                     .to_owned();
    ///
    /// let cte_referencing = SelectStatement::new()
    ///                             .column("id")
    ///                             .expr(Expr::col("depth").add(1i32))
    ///                             .column("next")
    ///                             .column("value")
    ///                             .from("task")
    ///                             .join(
    ///                                 JoinType::InnerJoin,
    ///                                 "cte_traversal",
    ///                                 Expr::col(("cte_traversal", "next")).equals(("task", "id"))
    ///                             )
    ///                             .to_owned();
    ///
    /// let common_table_expression = CommonTableExpression::new()
    ///             .query(
    ///                 base_query.clone().union(UnionType::All, cte_referencing).to_owned()
    ///             )
    ///             .columns(["id", "depth", "next", "value"])
    ///             .table_name("cte_traversal")
    ///             .to_owned();
    ///
    /// let with_clause = WithClause::new()
    ///         .recursive(true)
    ///         .cte(common_table_expression)
    ///         .to_owned();
    ///
    /// let query = SelectStatement::new()
    ///         .column(Asterisk)
    ///         .from("cte_traversal")
    ///         .with_cte(with_clause)
    ///         .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, 1, `next`, `value` FROM `task` UNION ALL (SELECT `id`, `depth` + 1, `next`, `value` FROM `task` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `task`.`id`)) SELECT * FROM `cte_traversal`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "task" UNION ALL (SELECT "id", "depth" + 1, "next", "value" FROM "task" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "task"."id")) SELECT * FROM "cte_traversal""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"WITH RECURSIVE "cte_traversal" ("id", "depth", "next", "value") AS (SELECT "id", 1, "next", "value" FROM "task" UNION ALL SELECT "id", "depth" + 1, "next", "value" FROM "task" INNER JOIN "cte_traversal" ON "cte_traversal"."next" = "task"."id") SELECT * FROM "cte_traversal""#
    /// );
    /// ```
    pub fn with_cte<C: Into<WithClause>>(&mut self, clause: C) -> &mut Self {
        self.with = Some(Box::new(clause.into()));
        self
    }

    /// WINDOW
    ///
    /// # Examples:
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
    pub fn window<A>(&mut self, name: A, window: WindowStatement) -> &mut Self
    where
        A: IntoIden,
    {
        self.window = Some((name.into_iden(), window));
        self
    }
}
