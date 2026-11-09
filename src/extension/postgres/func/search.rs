use super::*;

impl PgFunc {
    /// Call `TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::to_tsquery("a & b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT TO_TSQUERY('a & b')"#
    /// );
    /// ```
    pub fn to_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        text_search(PgFunc::ToTsquery, expr.into(), regconfig)
    }

    /// Call `TO_TSVECTOR` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::to_tsvector("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT TO_TSVECTOR('a b')"#
    /// );
    /// ```
    pub fn to_tsvector<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        text_search(PgFunc::ToTsvector, expr.into(), regconfig)
    }

    /// Call `PHRASE_TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::phraseto_tsquery("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PHRASETO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn phraseto_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        text_search(PgFunc::PhrasetoTsquery, expr.into(), regconfig)
    }

    /// Call `PLAIN_TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::plainto_tsquery("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT PLAINTO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn plainto_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        text_search(PgFunc::PlaintoTsquery, expr.into(), regconfig)
    }

    /// Call `WEBSEARCH_TO_TSQUERY` function. Postgres only.
    ///
    /// The parameter `regconfig` represents the OID of the text search configuration.
    /// If the value is `None` the argument is omitted from the query, and hence the database default used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::websearch_to_tsquery("a b", None))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT WEBSEARCH_TO_TSQUERY('a b')"#
    /// );
    /// ```
    pub fn websearch_to_tsquery<T>(expr: T, regconfig: Option<u32>) -> FunctionCall
    where
        T: Into<Expr>,
    {
        text_search(PgFunc::WebsearchToTsquery, expr.into(), regconfig)
    }

    /// Call `TS_RANK` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::ts_rank("a b", "a&b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT TS_RANK('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank<T>(vector: T, query: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TsRank).args([vector.into(), query.into()])
    }

    /// Call `TS_RANK_CD` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .expr(PgFunc::ts_rank_cd("a b", "a&b"))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT TS_RANK_CD('a b', 'a&b')"#
    /// );
    /// ```
    pub fn ts_rank_cd<T>(vector: T, query: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::TsRankCd).args([vector.into(), query.into()])
    }

    /// Call `ANY` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(PgFunc::any(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ANY(ARRAY [0,1])"#
    /// );
    /// ```
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select()
    ///     .column("id")
    ///     .from("post")
    ///     .and_where(Expr::col("tag").eq(PgFunc::any(Expr::cust("string_to_array('a,b,c', ',')"))))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT "id" FROM "post" WHERE "tag" = ANY(string_to_array('a,b,c', ','))"#
    /// );
    /// ```
    pub fn any<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::Any).arg(expr)
    }

    /// Call `SOME` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(PgFunc::some(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT SOME(ARRAY [0,1])"#
    /// );
    /// ```
    pub fn some<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::Some).arg(expr)
    }

    /// Call `ALL` function. Postgres only.
    ///
    /// # Examples
    ///
    /// ```
    /// use sql_builder::*;
    ///
    /// let query = Query::select().expr(PgFunc::all(vec![0, 1])).to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder).unwrap(),
    ///     r#"SELECT ALL(ARRAY [0,1])"#
    /// );
    /// ```
    pub fn all<T>(expr: T) -> FunctionCall
    where
        T: Into<Expr>,
    {
        FunctionCall::new(PgFunc::All).arg(expr)
    }
}

/// Calls `func` on `expr`, with the text search configuration `regconfig`
/// first when one is given.
fn text_search(func: PgFunc, expr: Expr, regconfig: Option<u32>) -> FunctionCall {
    match regconfig {
        Some(config) => FunctionCall::new(func).args([Expr::Value(config.into()), expr]),
        None => FunctionCall::new(func).arg(expr),
    }
}
