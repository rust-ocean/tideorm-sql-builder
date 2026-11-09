use super::*;

impl SelectStatement {
    /// Construct a new [`SelectStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    /// A select written as SQL text, with the values it binds in place, such
    /// as an operand or a CTE body the caller wrote itself. It is rendered as
    /// it is, so no other clause is set on it.
    pub fn custom(sql: BoundSql) -> Self {
        Self {
            custom: Some(sql),
            ..Self::default()
        }
    }

    /// Take the ownership of data in the current [`SelectStatement`]
    pub fn take(&mut self) -> Self {
        Self {
            distinct: self.distinct.take(),
            selects: std::mem::take(&mut self.selects),
            into: std::mem::take(&mut self.into),
            from: std::mem::take(&mut self.from),
            join: std::mem::take(&mut self.join),
            r#where: std::mem::replace(&mut self.r#where, ConditionHolder::new()),
            groups: std::mem::take(&mut self.groups),
            having: std::mem::replace(&mut self.having, ConditionHolder::new()),
            unions: std::mem::take(&mut self.unions),
            orders: std::mem::take(&mut self.orders),
            limit: self.limit.take(),
            offset: self.offset.take(),
            lock: self.lock.take(),
            window: self.window.take(),
            with: self.with.take(),
            custom: self.custom.take(),
        }
    }

    /// A shorthand to express if ... else ... when constructing the select statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .conditions(
    ///         true,
    ///         |x| {
    ///             x.and_where(Expr::col("font_id").eq(5));
    ///         },
    ///         |x| {
    ///             x.and_where(Expr::col("font_id").eq(10));
    ///         },
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn conditions<T, F>(&mut self, b: bool, if_true: T, if_false: F) -> &mut Self
    where
        T: FnOnce(&mut Self),
        F: FnOnce(&mut Self),
    {
        if b {
            if_true(self)
        } else {
            if_false(self)
        }
        self
    }

    /// A shorthand to express if ... else ... when constructing the select statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .apply_if(Some(5), |q, v| {
    ///         q.and_where(Expr::col("font_id").eq(v));
    ///     })
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// ```
    pub fn apply_if<T, F>(&mut self, val: Option<T>, if_some: F) -> &mut Self
    where
        Self: Sized,
        F: FnOnce(&mut Self, T),
    {
        if let Some(val) = val {
            if_some(self, val);
        }
        self
    }

    /// Construct part of the select statement in another function.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let common_expr = |q: &mut SelectStatement| {
    ///     q.and_where(Expr::col("font_id").eq(5));
    /// };
    ///
    /// let query = Query::select()
    ///     .column("character")
    ///     .from("character")
    ///     .apply(common_expr)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `character` FROM `character` WHERE `font_id` = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    /// );
    /// ```
    pub fn apply<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        func(self);
        self
    }
}
