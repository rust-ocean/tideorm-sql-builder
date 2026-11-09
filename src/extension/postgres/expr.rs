use super::PgBinOper;
use crate::{Expr, ExprTrait, IntoLikeExpr};

/// Postgres-specific operator methods for building expressions.
pub trait PgExpr: ExprTrait {
    /// Express an postgres concatenate (`||`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{extension::postgres::PgExpr, *};
    ///
    /// let query = Query::select().expr(Expr::val("a").concatenate("b")).take();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT 'a' || 'b'"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(Expr::val(vec!["a".to_owned()]).concatenate(vec!["b".to_owned()]))
    ///     .take();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ARRAY ['a'] || ARRAY ['b']"#
    /// );
    /// ```
    fn concatenate<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Concatenate, right)
    }

    /// Alias of [`PgExpr::concatenate`]
    fn concat<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.concatenate(right)
    }

    /// Express an postgres fulltext search matches (`@@`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns(["name", "variant", "language"])
    ///     .from("font")
    ///     .and_where(Expr::val("a & b").matches("a b"))
    ///     .and_where(Expr::col("name").matches("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' @@ 'a b' AND "name" @@ 'a b'"#
    /// );
    /// ```
    fn matches<T>(self, expr: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Matches, expr)
    }

    /// Express an postgres fulltext search contains (`@>`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns(["name", "variant", "language"])
    ///     .from("font")
    ///     .and_where(Expr::val("a & b").contains("a b"))
    ///     .and_where(Expr::col("name").contains("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' @> 'a b' AND "name" @> 'a b'"#
    /// );
    /// ```
    fn contains<T>(self, expr: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Contains, expr)
    }

    /// Express an postgres fulltext search contained (`<@`) expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns(["name", "variant", "language"])
    ///     .from("font")
    ///     .and_where(Expr::val("a & b").contained("a b"))
    ///     .and_where(Expr::col("name").contained("a b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "name", "variant", "language" FROM "font" WHERE 'a & b' <@ 'a b' AND "name" <@ 'a b'"#
    /// );
    /// ```
    fn contained<T>(self, expr: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::Contained, expr)
    }

    /// Express a `ILIKE` expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{*, extension::postgres::PgExpr};
    ///
    /// let query = Query::select()
    ///     .columns(["character", "size_w", "size_h"])
    ///     .from("character")
    ///     .and_where(Expr::col(("character", "character")).ilike("Ours'%"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "character"."character" ILIKE E'Ours\'%'"#
    /// );
    /// ```
    fn ilike<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        self.binary(PgBinOper::ILike, like.into_like_expr())
    }

    /// Express a `NOT ILIKE` expression
    fn not_ilike<L>(self, like: L) -> Expr
    where
        L: IntoLikeExpr,
    {
        self.binary(PgBinOper::NotILike, like.into_like_expr())
    }

    /// Express a postgres retrieves JSON field as JSON value (`->`).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{extension::postgres::PgExpr, *};
    ///
    /// let query = Query::select()
    ///     .column("variant")
    ///     .from("font")
    ///     .and_where(Expr::col("variant").get_json_field("a"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "variant" FROM "font" WHERE "variant" -> 'a'"#
    /// );
    /// ```
    fn get_json_field<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::GetJsonField, right)
    }

    /// Express a postgres retrieves JSON field and casts it to an appropriate SQL type (`->>`).
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::{extension::postgres::PgExpr, *};
    ///
    /// let query = Query::select()
    ///     .column("variant")
    ///     .from("font")
    ///     .and_where(Expr::col("variant").cast_json_field("a"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "variant" FROM "font" WHERE "variant" ->> 'a'"#
    /// );
    /// ```
    fn cast_json_field<T>(self, right: T) -> Expr
    where
        T: Into<Expr>,
    {
        self.binary(PgBinOper::CastJsonField, right)
    }

    /// Equivalent to `is_in` but bind parameters as array.
    /// ```
    /// use sql_builder::{extension::postgres::PgExpr, *};
    ///
    /// let query = Query::select()
    ///     .columns(["id"])
    ///     .from("character")
    ///     .and_where(
    ///         ("character", "size_w")
    ///             .into_column_ref()
    ///             .eq_any([1, 2, 3]),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" = ANY(ARRAY [1,2,3])"#
    /// );
    /// ```
    fn eq_any<V, I>(self, v: I) -> Expr
    where
        V: Into<crate::Value> + crate::ValueType,
        I: IntoIterator<Item = V>,
    {
        self.eq(super::PgFunc::any(crate::Value::Array(
            V::array_type(),
            Some(Box::new(v.into_iter().map(|v| v.into()).collect())),
        )))
    }

    /// Equivalent to `is_not_in` but bind parameters as array.
    /// ```
    /// use sql_builder::{extension::postgres::PgExpr, *};
    ///
    /// let query = Query::select()
    ///     .columns(["id"])
    ///     .from("character")
    ///     .and_where(
    ///         ("character", "size_w")
    ///             .into_column_ref()
    ///             .ne_all([1, 2, 3]),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "character" WHERE "character"."size_w" <> ALL(ARRAY [1,2,3])"#
    /// );
    /// ```
    fn ne_all<V, I>(self, v: I) -> Expr
    where
        V: Into<crate::Value> + crate::ValueType,
        I: IntoIterator<Item = V>,
    {
        self.ne(super::PgFunc::all(crate::Value::Array(
            V::array_type(),
            Some(Box::new(v.into_iter().map(|v| v.into()).collect())),
        )))
    }
}

/// You should be able to use Postgres-specific operators with all types of expressions.
impl<T> PgExpr for T where T: ExprTrait {}
