use std::borrow::Cow;

use crate::{Expr, IntoColumnRef, NullOrdering, Order, OrderExpr};

pub trait OrderedStatement {
    #[doc(hidden)]
    // Implementation for the trait.
    fn add_order_by(&mut self, order: OrderExpr) -> &mut Self;

    /// Clear order expressions
    fn clear_order_by(&mut self) -> &mut Self;

    /// Order by column.
    ///
    /// # Examples
    ///
    /// Order by ASC and DESC
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("aspect")
    ///     .from("glyph")
    ///     .and_where(Expr::col("aspect").if_null(0).gt(2))
    ///     .order_by("image", Order::Desc)
    ///     .order_by(("glyph", "aspect"), Order::Asc)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    ///
    /// Order by custom field ordering
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .columns(["aspect"])
    ///     .from("glyph")
    ///     .order_by(
    ///         "id",
    ///         Order::Field(Values(vec![4.into(), 5.into(), 1.into(), 3.into()])),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     [
    ///         r#"SELECT `aspect`"#,
    ///         r#"FROM `glyph`"#,
    ///         r#"ORDER BY CASE"#,
    ///         r#"WHEN `id`=4 THEN 0"#,
    ///         r#"WHEN `id`=5 THEN 1"#,
    ///         r#"WHEN `id`=1 THEN 2"#,
    ///         r#"WHEN `id`=3 THEN 3"#,
    ///         r#"ELSE 4 END"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     [
    ///         r#"SELECT "aspect""#,
    ///         r#"FROM "glyph""#,
    ///         r#"ORDER BY CASE"#,
    ///         r#"WHEN "id"=4 THEN 0"#,
    ///         r#"WHEN "id"=5 THEN 1"#,
    ///         r#"WHEN "id"=1 THEN 2"#,
    ///         r#"WHEN "id"=3 THEN 3"#,
    ///         r#"ELSE 4 END"#,
    ///     ]
    ///     .join(" ")
    /// );
    ///
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     [
    ///         r#"SELECT "aspect""#,
    ///         r#"FROM "glyph""#,
    ///         r#"ORDER BY CASE"#,
    ///         r#"WHEN "id"=4 THEN 0"#,
    ///         r#"WHEN "id"=5 THEN 1"#,
    ///         r#"WHEN "id"=1 THEN 2"#,
    ///         r#"WHEN "id"=3 THEN 3"#,
    ///         r#"ELSE 4 END"#,
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_order_by(OrderExpr {
            expr: Expr::Column(col.into_column_ref()),
            order,
            nulls: None,
        })
    }

    /// Order by [`Expr`].
    fn order_by_expr(&mut self, expr: Expr, order: Order) -> &mut Self {
        self.add_order_by(OrderExpr {
            expr,
            order,
            nulls: None,
        })
    }

    /// Order by custom string.
    fn order_by_customs<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (T, Order)>,
    {
        cols.into_iter().for_each(|(c, order)| {
            self.add_order_by(OrderExpr {
                expr: Expr::Custom(c.into()),
                order,
                nulls: None,
            });
        });
        self
    }

    /// Order by vector of columns.
    fn order_by_columns<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order)>,
    {
        cols.into_iter().for_each(|(c, order)| {
            self.add_order_by(OrderExpr {
                expr: Expr::Column(c.into_column_ref()),
                order,
                nulls: None,
            });
        });
        self
    }

    /// Order by column with nulls order option.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("aspect")
    ///     .from("glyph")
    ///     .order_by_with_nulls("image", Order::Desc, NullOrdering::Last)
    ///     .order_by_with_nulls(("glyph", "aspect"), Order::Asc, NullOrdering::First)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "aspect" FROM "glyph" ORDER BY "image" DESC NULLS LAST, "glyph"."aspect" ASC NULLS FIRST"#
    /// );
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `aspect` FROM `glyph` ORDER BY `image` IS NULL ASC, `image` DESC, `glyph`.`aspect` IS NULL DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    fn order_by_with_nulls<T>(&mut self, col: T, order: Order, nulls: NullOrdering) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_order_by(OrderExpr {
            expr: Expr::Column(col.into_column_ref()),
            order,
            nulls: Some(nulls),
        })
    }

    /// Order by [`Expr`] with nulls order option.
    fn order_by_expr_with_nulls(
        &mut self,
        expr: Expr,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self {
        self.add_order_by(OrderExpr {
            expr,
            order,
            nulls: Some(nulls),
        })
    }

    /// Order by custom string with nulls order option.
    fn order_by_customs_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = (T, Order, NullOrdering)>,
    {
        cols.into_iter().for_each(|(c, order, nulls)| {
            self.add_order_by(OrderExpr {
                expr: Expr::Custom(c.into()),
                order,
                nulls: Some(nulls),
            });
        });
        self
    }

    /// Order by vector of columns with nulls order option.
    fn order_by_columns_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order, NullOrdering)>,
    {
        cols.into_iter().for_each(|(c, order, nulls)| {
            self.add_order_by(OrderExpr {
                expr: Expr::Column(c.into_column_ref()),
                order,
                nulls: Some(nulls),
            });
        });
        self
    }
}

/// Implements [`OrderedStatement`] over the statement's `Vec<OrderExpr>` field `$orders`, with
/// inherent copies of the trait's methods so callers need not import it.
macro_rules! ordered_statement {
    ($statement:ty, $orders:ident) => {
        impl $crate::OrderedStatement for $statement {
            fn add_order_by(&mut self, order: $crate::OrderExpr) -> &mut Self {
                self.$orders.push(order);
                self
            }

            fn clear_order_by(&mut self) -> &mut Self {
                self.$orders = Vec::new();
                self
            }
        }

        impl $statement {
            pub fn add_order_by(&mut self, order: $crate::OrderExpr) -> &mut Self {
                <Self as $crate::OrderedStatement>::add_order_by(self, order)
            }

            pub fn clear_order_by(&mut self) -> &mut Self {
                <Self as $crate::OrderedStatement>::clear_order_by(self)
            }

            pub fn order_by<T: $crate::IntoColumnRef>(
                &mut self,
                col: T,
                order: $crate::Order,
            ) -> &mut Self {
                <Self as $crate::OrderedStatement>::order_by(self, col, order)
            }

            pub fn order_by_expr(&mut self, expr: $crate::Expr, order: $crate::Order) -> &mut Self {
                <Self as $crate::OrderedStatement>::order_by_expr(self, expr, order)
            }

            pub fn order_by_customs<I, T>(&mut self, cols: I) -> &mut Self
            where
                T: Into<::std::borrow::Cow<'static, str>>,
                I: IntoIterator<Item = (T, $crate::Order)>,
            {
                <Self as $crate::OrderedStatement>::order_by_customs(self, cols)
            }

            pub fn order_by_columns<I, T>(&mut self, cols: I) -> &mut Self
            where
                T: $crate::IntoColumnRef,
                I: IntoIterator<Item = (T, $crate::Order)>,
            {
                <Self as $crate::OrderedStatement>::order_by_columns(self, cols)
            }

            pub fn order_by_with_nulls<T: $crate::IntoColumnRef>(
                &mut self,
                col: T,
                order: $crate::Order,
                nulls: $crate::NullOrdering,
            ) -> &mut Self {
                <Self as $crate::OrderedStatement>::order_by_with_nulls(self, col, order, nulls)
            }

            pub fn order_by_expr_with_nulls(
                &mut self,
                expr: $crate::Expr,
                order: $crate::Order,
                nulls: $crate::NullOrdering,
            ) -> &mut Self {
                <Self as $crate::OrderedStatement>::order_by_expr_with_nulls(
                    self, expr, order, nulls,
                )
            }

            pub fn order_by_customs_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
            where
                T: Into<::std::borrow::Cow<'static, str>>,
                I: IntoIterator<Item = (T, $crate::Order, $crate::NullOrdering)>,
            {
                <Self as $crate::OrderedStatement>::order_by_customs_with_nulls(self, cols)
            }

            pub fn order_by_columns_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
            where
                T: $crate::IntoColumnRef,
                I: IntoIterator<Item = (T, $crate::Order, $crate::NullOrdering)>,
            {
                <Self as $crate::OrderedStatement>::order_by_columns_with_nulls(self, cols)
            }
        }
    };
}
pub(crate) use ordered_statement;
