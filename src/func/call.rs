use super::*;

/// Function call.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub(crate) func: Func,
    pub(crate) args: Vec<Expr>,
    pub(crate) mods: Vec<FuncArgMod>,
    pub(crate) filter: Option<Condition>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FuncArgMod {
    pub distinct: bool,
}

impl FunctionCall {
    pub(crate) fn new(func: impl Into<Func>) -> Self {
        Self {
            func: func.into(),
            args: Vec::new(),
            mods: Vec::new(),
            filter: None,
        }
    }

    /// Append an argument to the function call
    pub fn arg<T>(self, arg: T) -> Self
    where
        T: Into<Expr>,
    {
        self.arg_with(arg, Default::default())
    }

    pub(crate) fn arg_with<T>(mut self, arg: T, mod_: FuncArgMod) -> Self
    where
        T: Into<Expr>,
    {
        self.args.push(arg.into());
        self.mods.push(mod_);
        self
    }

    /// Replace the arguments of the function call
    pub fn args<I>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = Expr>,
    {
        self.args = args.into_iter().collect();
        self.mods = vec![Default::default(); self.args.len()];
        self
    }

    pub fn get_func(&self) -> &Func {
        &self.func
    }

    pub fn get_args(&self) -> &[Expr] {
        &self.args
    }

    pub fn get_mods(&self) -> &[FuncArgMod] {
        &self.mods
    }

    /// Append a `FILTER (WHERE ...)` clause to the function call.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Func::count(Expr::val(1))
    ///             .filter(
    ///                 Cond::all()
    ///                     .add(Expr::col("character").eq("foo"))
    ///                     .add(Expr::col("size_w").eq(1))
    ///             ),
    ///         Alias::new("filtered_total")
    ///     )
    ///     .expr_as(Func::count(Expr::val(1)), Alias::new("total"))
    ///     .from("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT(1) FILTER (WHERE "character" = 'foo' AND "size_w" = 1) AS "filtered_total", COUNT(1) AS "total" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder).unwrap(),
    ///     r#"SELECT COUNT(1) FILTER (WHERE "character" = 'foo' AND "size_w" = 1) AS "filtered_total", COUNT(1) AS "total" FROM "character""#
    /// );
    /// ```
    pub fn filter(mut self, condition: impl Into<Condition>) -> Self {
        self.filter = Some(condition.into());
        self
    }

    pub fn get_filter(&self) -> Option<&Condition> {
        self.filter.as_ref()
    }
}
