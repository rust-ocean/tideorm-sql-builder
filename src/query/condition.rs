mod conditional;
mod constant;
mod holder;

pub use conditional::*;

use crate::{ExprTrait, expr::Expr, types::LogicalChainOper};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionType {
    Any,
    All,
}

/// Represents the value of an [`Condition::any`] or [`Condition::all`]: a set of disjunctive or conjunctive conditions.
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub(crate) negate: bool,
    pub(crate) condition_type: ConditionType,
    pub(crate) conditions: Vec<ConditionExpression>,
}

pub type Cond = Condition;

impl From<Expr> for Condition {
    fn from(expr: Expr) -> Self {
        Condition {
            negate: false,
            condition_type: ConditionType::All,
            conditions: vec![ConditionExpression::Expr(expr)],
        }
    }
}

impl From<ConditionExpression> for Condition {
    fn from(ce: ConditionExpression) -> Self {
        Condition {
            negate: false,
            condition_type: ConditionType::All,
            conditions: vec![ce],
        }
    }
}

/// A helper trait.
///
/// You shouldn't implement this manually.
pub trait IntoCondition: Into<Condition> {
    #[inline(always)]
    fn into_condition(self) -> Condition {
        self.into()
    }
}

impl<T> IntoCondition for T where T: Into<Condition> {}

/// An internal representation of conditions, either an expression or a nested condition.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConditionExpression {
    Condition(Condition),
    Expr(Expr),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConditionHolderContents {
    #[default]
    Empty,
    Chain(Vec<LogicalChainOper>),
    Condition(Condition),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConditionHolder {
    pub contents: ConditionHolderContents,
}

impl Condition {
    /// Add a condition to the set.
    ///
    /// If it's an [`Condition::any`], it will be separated from the others by an `" OR "` in the query. If it's
    /// an [`Condition::all`], it will be separated by an `" AND "`.
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let statement = Query::select()
    ///     .column("id")
    ///     .from("glyph")
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(Expr::col("aspect").eq(0).into_condition().not())
    ///             .add(Expr::col("id").eq(0).into_condition().not()),
    ///     )
    ///     .to_string(PostgresQueryBuilder).unwrap();
    /// assert_eq!(
    ///     statement,
    ///     r#"SELECT "id" FROM "glyph" WHERE (NOT "aspect" = 0) AND (NOT "id" = 0)"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add<C>(mut self, condition: C) -> Self
    where
        C: Into<Condition>,
    {
        let condition: Condition = condition.into();
        let condition: ConditionExpression = condition.into();
        self.conditions.push(condition);
        self
    }

    /// Add an optional condition to the set.
    ///
    /// Shorthand for `if o.is_some() { self.add(o) }`
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
    ///             .add_option(Some(Expr::col(("glyph", "image")).like("A%")))
    ///             .add_option(None::<Expr>),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add_option<C>(self, other: Option<C>) -> Self
    where
        C: Into<Condition>,
    {
        if let Some(other) = other {
            self.add(other)
        } else {
            self
        }
    }

    /// Create a condition that is true if any of the conditions is true.
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
    ///         Cond::any()
    ///             .add(Expr::col(("glyph", "aspect")).is_in([3, 4]))
    ///             .add(Expr::col(("glyph", "image")).like("A%"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    pub fn any() -> Condition {
        Condition {
            negate: false,
            condition_type: ConditionType::Any,
            conditions: Vec::new(),
        }
    }

    /// Create a condition that is false if any of the conditions is false.
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
    ///             .add(Expr::col(("glyph", "image")).like("A%"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
    /// );
    /// ```
    pub fn all() -> Condition {
        Condition {
            negate: false,
            condition_type: ConditionType::All,
            conditions: Vec::new(),
        }
    }

    /// Negates a condition.
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
    ///             .not()
    ///             .add(Expr::col(("glyph", "aspect")).is_in([3, 4]))
    ///             .add(Expr::col(("glyph", "image")).like("A%"))
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `image` FROM `glyph` WHERE NOT (`glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%')"#
    /// );
    /// ```
    ///
    /// # More Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("id")
    ///     .cond_where(
    ///         Cond::all()
    ///             .add(
    ///                 Cond::all()
    ///                     .not()
    ///                     .add(Expr::val(1).eq(1))
    ///                     .add(Expr::val(2).eq(2)),
    ///             )
    ///             .add(Cond::any().add(Expr::val(3).eq(3)).add(Expr::val(4).eq(4))),
    ///     )
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder).unwrap(),
    ///     r#"SELECT `id` WHERE (NOT (1 = 1 AND 2 = 2)) AND (3 = 3 OR 4 = 4)"#
    /// );
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn not(mut self) -> Self {
        self.negate = !self.negate;
        self
    }

    /// Whether or not any condition has been added
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let is_empty = Cond::all().is_empty();
    ///
    /// assert!(is_empty);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    /// How many conditions were added
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let len = Cond::all().len();
    ///
    /// assert_eq!(len, 0);
    /// ```
    pub fn len(&self) -> usize {
        self.conditions.len()
    }
}

impl From<Condition> for Expr {
    fn from(cond: Condition) -> Self {
        let mut inner_exprs = vec![];
        for ce in cond.conditions {
            inner_exprs.push(match ce {
                ConditionExpression::Condition(c) => c.into(),
                ConditionExpression::Expr(e) => e,
            });
        }
        let mut inner_exprs_into_iter = inner_exprs.into_iter();
        let expr = if let Some(first_expr) = inner_exprs_into_iter.next() {
            let mut out_expr = first_expr;
            for e in inner_exprs_into_iter {
                out_expr = match cond.condition_type {
                    ConditionType::Any => out_expr.or(e),
                    ConditionType::All => out_expr.and(e),
                };
            }
            out_expr
        } else {
            Expr::Constant(match cond.condition_type {
                ConditionType::Any => false.into(),
                ConditionType::All => true.into(),
            })
        };
        if cond.negate { expr.not() } else { expr }
    }
}

impl From<ConditionExpression> for Expr {
    fn from(ce: ConditionExpression) -> Self {
        match ce {
            ConditionExpression::Condition(c) => c.into(),
            ConditionExpression::Expr(e) => e,
        }
    }
}

impl From<Condition> for ConditionExpression {
    fn from(condition: Condition) -> Self {
        ConditionExpression::Condition(condition)
    }
}

impl From<Expr> for ConditionExpression {
    fn from(condition: Expr) -> Self {
        ConditionExpression::Expr(condition)
    }
}

/// Macro to easily create an [`Condition::any`].
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
///         any![
///             Expr::col(("glyph", "aspect")).is_in([3, 4]),
///             Expr::col(("glyph", "image")).like("A%")
///         ]
///     )
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) OR `glyph`.`image` LIKE 'A%'"#
/// );
/// ```
#[macro_export]
macro_rules! any {
    ( $( $x:expr ),* $(,)?) => {
        {
            let mut tmp = $crate::Condition::any();
            $(
                tmp = tmp.add($x);
            )*
            tmp
        }
    };
}

/// Macro to easily create an [`Condition::all`].
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
///         all![
///             Expr::col(("glyph", "aspect")).is_in([3, 4]),
///             Expr::col(("glyph", "image")).like("A%")
///         ]
///     )
///     .to_owned();
///
/// assert_eq!(
///     query.to_string(MysqlQueryBuilder).unwrap(),
///     r#"SELECT `image` FROM `glyph` WHERE `glyph`.`aspect` IN (3, 4) AND `glyph`.`image` LIKE 'A%'"#
/// );
#[macro_export]
macro_rules! all {
    ( $( $x:expr ),* $(,)?) => {
        {
            let mut tmp = $crate::Condition::all();
            $(
                tmp = tmp.add($x);
            )*
            tmp
        }
    };
}
