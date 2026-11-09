use super::*;

pub trait ConditionalStatement {
    /// And where condition.
    /// Calling `or_where` after `and_where` will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("image")
    ///     .from("glyph")
    ///     .and_where(Expr::col(("glyph", "aspect")).is_in([3, 4]))
    ///     .and_where(Expr::col(("glyph", "image")).like("A%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    fn and_where(&mut self, other: Expr) -> &mut Self {
        self.cond_where(other)
    }

    /// Optional and where, short hand for `if c.is_some() q.and_where(c)`.
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("image")
    ///     .from("glyph")
    ///     .and_where(Expr::col("aspect").is_in([3, 4]))
    ///     .and_where_option(Some(Expr::col("image").like("A%")))
    ///     .and_where_option(None)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM `glyph` WHERE `aspect` IN (3, 4) AND `image` LIKE 'A%'"#
    /// );
    /// ```
    fn and_where_option(&mut self, other: Option<Expr>) -> &mut Self {
        if let Some(other) = other {
            self.and_where(other);
        }
        self
    }

    #[doc(hidden)]
    // Trait implementation.
    fn and_or_where(&mut self, condition: LogicalChainOper) -> &mut Self;

    /// Where condition, expressed with `any` and `all`.
    /// Calling `cond_where` multiple times will conjoin them.
    /// Calling `or_where` after `cond_where` will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("image")
    ///     .from("glyph")
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(Expr::col(("glyph", "aspect")).is_in([3, 4]))
    ///             .add(Cond::any()
    ///                 .add(Expr::col(("glyph", "image")).like("A%"))
    ///                 .add(Expr::col(("glyph", "image")).like("B%"))
    ///             )
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4) AND ("glyph"."image" LIKE 'A%' OR "glyph"."image" LIKE 'B%')"#
    /// );
    /// ```
    ///
    /// Using macro
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("image")
    ///     .from("glyph")
    ///     .cond_where(
    ///         all![
    ///             Expr::col(("glyph", "aspect")).is_in([3, 4]),
    ///             any![
    ///                 Expr::col(("glyph", "image")).like("A%"),
    ///                 Expr::col(("glyph", "image")).like("B%"),
    ///             ]
    ///         ])
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4) AND ("glyph"."image" LIKE 'A%' OR "glyph"."image" LIKE 'B%')"#
    /// );
    /// ```
    ///
    /// Calling multiple times; the following two are equivalent:
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column("id")
    ///         .from("glyph")
    ///         .cond_where(Expr::col("id").eq(1))
    ///         .cond_where(any![Expr::col("id").eq(2), Expr::col("id").eq(3)])
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "glyph" WHERE "id" = 1 AND ("id" = 2 OR "id" = 3)"#
    /// );
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column("id")
    ///         .from("glyph")
    ///         .cond_where(any![Expr::col("id").eq(2), Expr::col("id").eq(3)])
    ///         .cond_where(Expr::col("id").eq(1))
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "glyph" WHERE ("id" = 2 OR "id" = 3) AND "id" = 1"#
    /// );
    /// ```
    ///
    /// Calling multiple times; will be ANDed togother
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column("id")
    ///         .from("glyph")
    ///         .cond_where(any![Expr::col("id").eq(1), Expr::col("id").eq(2)])
    ///         .cond_where(any![Expr::col("id").eq(3), Expr::col("id").eq(4)])
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "glyph" WHERE ("id" = 1 OR "id" = 2) AND ("id" = 3 OR "id" = 4)"#
    /// );
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column("id")
    ///         .from("glyph")
    ///         .cond_where(all![Expr::col("id").eq(1), Expr::col("id").eq(2)])
    ///         .cond_where(all![Expr::col("id").eq(3), Expr::col("id").eq(4)])
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "glyph" WHERE "id" = 1 AND "id" = 2 AND "id" = 3 AND "id" = 4"#
    /// );
    /// ```
    ///
    /// Some more test cases involving negation
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column("id")
    ///         .from("glyph")
    ///         .cond_where(
    ///             Cond::all()
    ///                 .not()
    ///                 .add(Expr::col("id").eq(1))
    ///                 .add(Expr::col("id").eq(2)),
    ///         )
    ///         .cond_where(
    ///             Cond::all()
    ///                 .add(Expr::col("id").eq(3))
    ///                 .add(Expr::col("id").eq(4)),
    ///         )
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "glyph" WHERE (NOT ("id" = 1 AND "id" = 2)) AND ("id" = 3 AND "id" = 4)"#
    /// );
    ///
    /// assert_eq!(
    ///     Query::select()
    ///         .column("id")
    ///         .from("glyph")
    ///         .cond_where(
    ///             Cond::all()
    ///                 .add(Expr::col("id").eq(3))
    ///                 .add(Expr::col("id").eq(4)),
    ///         )
    ///         .cond_where(
    ///             Cond::all()
    ///                 .not()
    ///                 .add(Expr::col("id").eq(1))
    ///                 .add(Expr::col("id").eq(2)),
    ///         )
    ///         .to_owned()
    ///         .to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "glyph" WHERE "id" = 3 AND "id" = 4 AND (NOT ("id" = 1 AND "id" = 2))"#
    /// );
    /// ```
    fn cond_where<C>(&mut self, condition: C) -> &mut Self
    where
        C: IntoCondition;
}

/// Implements [`ConditionalStatement`] over the statement's `r#where` holder, with inherent
/// copies of the trait's methods so callers need not import it.
macro_rules! conditional_statement {
    ($statement:ty) => {
        impl $crate::ConditionalStatement for $statement {
            fn and_or_where(&mut self, condition: $crate::LogicalChainOper) -> &mut Self {
                self.r#where.add_and_or(condition);
                self
            }

            fn cond_where<C>(&mut self, condition: C) -> &mut Self
            where
                C: $crate::IntoCondition,
            {
                self.r#where.add_condition(condition.into_condition());
                self
            }
        }

        impl $statement {
            pub fn and_where(&mut self, other: $crate::Expr) -> &mut Self {
                <Self as $crate::ConditionalStatement>::and_where(self, other)
            }

            pub fn and_where_option(&mut self, other: Option<$crate::Expr>) -> &mut Self {
                <Self as $crate::ConditionalStatement>::and_where_option(self, other)
            }

            pub fn and_or_where(&mut self, condition: $crate::LogicalChainOper) -> &mut Self {
                <Self as $crate::ConditionalStatement>::and_or_where(self, condition)
            }

            pub fn cond_where<C: $crate::IntoCondition>(&mut self, condition: C) -> &mut Self {
                <Self as $crate::ConditionalStatement>::cond_where(self, condition)
            }
        }
    };
}
pub(crate) use conditional_statement;
