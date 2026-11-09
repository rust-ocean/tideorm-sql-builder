use super::*;

impl PgFunc {
    /// Call `STARTS_WITH` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::starts_with("123", "1"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT STARTS_WITH('123', '1')"#
    /// );
    /// ```
    pub fn starts_with<T, P>(text: T, prefix: P) -> FunctionCall
    where
        T: Into<Expr>,
        P: Into<Expr>,
    {
        FunctionCall::new(PgFunc::StartsWith).args([text.into(), prefix.into()])
    }

    /// Call `GEN_RANDOM_UUID` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(PgFunc::gen_random_uuid()).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT GEN_RANDOM_UUID()"#
    /// );
    /// ```
    pub fn gen_random_uuid() -> FunctionCall {
        FunctionCall::new(PgFunc::GenRandomUUID)
    }

    /// Call the `JSON_BUILD_OBJECT` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::json_build_object(vec![
    ///         (Expr::val("a"), Expr::val(1)),
    ///         (Expr::val("b"), Expr::val("2")),
    ///     ]))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT JSON_BUILD_OBJECT('a', 1, 'b', '2')"#
    /// );
    /// ```
    pub fn json_build_object<T>(pairs: Vec<(T, T)>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        let mut args = vec![];
        for (key, value) in pairs {
            args.push(key.into());
            args.push(value.into());
        }
        FunctionCall::new(PgFunc::JsonBuildObject).args(args)
    }

    /// Call the `DATE_TRUNC` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::date_trunc(
    ///         PgDateTruncUnit::Day,
    ///         Expr::val("2020-01-01"),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT DATE_TRUNC('day', '2020-01-01')"#
    /// );
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::date_trunc(
    ///         PgDateTruncUnit::Microseconds,
    ///         Expr::val("2020-01-01"),
    ///     ))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT DATE_TRUNC('microseconds', '2020-01-01')"#
    /// );
    /// ```
    pub fn date_trunc<T>(unit: PgDateTruncUnit, expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::DateTrunc).args([Expr::val(unit.to_string()), expr.into()])
    }

    /// Call the `JSON_AGG` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr(PgFunc::json_agg(Expr::col("size_w")))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT JSON_AGG("size_w") FROM "character""#
    /// );
    /// ```
    pub fn json_agg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::JsonAgg).arg(expr)
    }

    /// Call the `ARRAY_AGG` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr(PgFunc::array_agg(Expr::col("id")))
    ///     .group_by_col("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ARRAY_AGG("id") FROM "character" GROUP BY "character""#
    /// );
    /// ```
    pub fn array_agg<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::ArrayAgg).arg(expr)
    }

    /// Call the `ARRAY_AGG` function with the `DISTINCT` modifier. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .from("character")
    ///     .expr(PgFunc::array_agg_distinct(Expr::col("id")))
    ///     .group_by_col("character")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ARRAY_AGG(DISTINCT "id") FROM "character" GROUP BY "character""#
    /// );
    /// ```
    pub fn array_agg_distinct<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::ArrayAgg).arg_with(expr, FuncArgMod { distinct: true })
    }

    /// Call `PG_ADVISORY_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_lock(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_LOCK(12345)"#
    /// );
    /// ```
    pub fn advisory_lock<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::AdvisoryLock).arg(key)
    }

    /// Call `PG_ADVISORY_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_lock_shared(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_LOCK_SHARED(12345)"#
    /// );
    /// ```
    pub fn advisory_lock_shared<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::AdvisoryLockShared).arg(key)
    }

    /// Call `PG_TRY_ADVISORY_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_lock(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_TRY_ADVISORY_LOCK(12345)"#
    /// );
    /// ```
    pub fn try_advisory_lock<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TryAdvisoryLock).arg(key)
    }

    /// Call `PG_TRY_ADVISORY_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_lock_shared(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_TRY_ADVISORY_LOCK_SHARED(12345)"#
    /// );
    /// ```
    pub fn try_advisory_lock_shared<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TryAdvisoryLockShared).arg(key)
    }

    /// Call `PG_ADVISORY_UNLOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_UNLOCK(12345)"#
    /// );
    /// ```
    pub fn advisory_unlock<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::AdvisoryUnlock).arg(key)
    }

    /// Call `PG_ADVISORY_UNLOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock_shared(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_UNLOCK_SHARED(12345)"#
    /// );
    /// ```
    pub fn advisory_unlock_shared<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::AdvisoryUnlockShared).arg(key)
    }

    /// Call `PG_ADVISORY_UNLOCK_ALL` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_unlock_all())
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_UNLOCK_ALL()"#
    /// );
    /// ```
    pub fn advisory_unlock_all() -> FunctionCall {
        FunctionCall::new(PgFunc::AdvisoryUnlockAll)
    }

    /// Call `PG_ADVISORY_XACT_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_xact_lock(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_XACT_LOCK(12345)"#
    /// );
    /// ```
    pub fn advisory_xact_lock<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::AdvisoryXactLock).arg(key)
    }

    /// Call `PG_ADVISORY_XACT_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::advisory_xact_lock_shared(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_ADVISORY_XACT_LOCK_SHARED(12345)"#
    /// );
    /// ```
    pub fn advisory_xact_lock_shared<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::AdvisoryXactLockShared).arg(key)
    }

    /// Call `PG_TRY_ADVISORY_XACT_LOCK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_xact_lock(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_TRY_ADVISORY_XACT_LOCK(12345)"#
    /// );
    /// ```
    pub fn try_advisory_xact_lock<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TryAdvisoryXactLock).arg(key)
    }

    /// Call `PG_TRY_ADVISORY_XACT_LOCK_SHARED` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::try_advisory_xact_lock_shared(Expr::val(12345_i64)))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PG_TRY_ADVISORY_XACT_LOCK_SHARED(12345)"#
    /// );
    /// ```
    pub fn try_advisory_xact_lock_shared<T>(key: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TryAdvisoryXactLockShared).arg(key)
    }
}
