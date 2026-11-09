use crate::{Condition, Expr, IntoCondition};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaseStatementCondition {
    pub(crate) condition: Condition,
    pub(crate) result: Expr,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaseStatement {
    pub(crate) when: Vec<CaseStatementCondition>,
    pub(crate) r#else: Option<Expr>,
}

impl CaseStatement {
    /// Creates a new case statement expression
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         CaseStatement::new()
    ///             .case(Expr::col(("glyph", "aspect")).is_in([2, 4]), true)
    ///             .finally(false),
    ///          "is_even"
    ///     )
    ///     .from("glyph")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///             Expr::case(
    ///                 Expr::col(("glyph", "aspect")).gt(0),
    ///                 "positive"
    ///              )
    ///             .case(
    ///                 Expr::col(("glyph", "aspect")).lt(0),
    ///                 "negative"
    ///              )
    ///             .finally("zero"),
    ///          "polarity"
    ///     )
    ///     .from("glyph")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" > 0) THEN 'positive' WHEN ("glyph"."aspect" < 0) THEN 'negative' ELSE 'zero' END) AS "polarity" FROM "glyph""#
    /// );
    /// ```
    pub fn case<C, T>(mut self, cond: C, then: T) -> Self
    where
        C: IntoCondition,
        T: Into<Expr>,
    {
        self.when.push(CaseStatementCondition {
            condition: cond.into_condition(),
            result: then.into(),
        });
        self
    }

    /// Ends the case statement with the final `ELSE` result.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///             Cond::any()
    ///                 .add(Expr::col(("character", "font_size")).gt(48))
    ///                 .add(Expr::col(("character", "size_w")).gt(500)),
    ///             "large"
    ///         )
    ///         .case(
    ///             Cond::any()
    ///                 .add(Expr::col(("character", "font_size")).between(24,48))
    ///                 .add(Expr::col(("character", "size_w")).between(300,500)),
    ///             "medium"
    ///         )
    ///         .finally("small"),
    ///         "char_size")
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     [
    ///         r#"SELECT"#,
    ///         r#"(CASE WHEN ("character"."font_size" > 48 OR "character"."size_w" > 500) THEN 'large'"#,
    ///         r#"WHEN (("character"."font_size" BETWEEN 24 AND 48) OR ("character"."size_w" BETWEEN 300 AND 500)) THEN 'medium'"#,
    ///         r#"ELSE 'small' END) AS "char_size""#,
    ///         r#"FROM "character""#
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn finally<E>(mut self, r#else: E) -> Self
    where
        E: Into<Expr>,
    {
        self.r#else = Some(r#else.into());
        self
    }
}

#[allow(clippy::from_over_into)]
impl Into<Expr> for CaseStatement {
    fn into(self) -> Expr {
        Expr::Case(Box::new(self))
    }
}

#[cfg(test)]
#[path = "../../tests/unit/case_tests.rs"]
mod tests;
