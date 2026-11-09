use super::*;

impl QueryBuilder for SqliteQueryBuilder {
    fn lexer_dialect(&self) -> crate::lexer::Dialect {
        crate::lexer::Dialect::Sqlite
    }

    fn prepare_expr(
        &self,
        simple_expr: &Expr,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        if matches!(simple_expr, Expr::Keyword(Keyword::Default)) {
            return Err(RenderError::Unsupported(
                "SQLite has no DEFAULT value; leave the column out of the INSERT".into(),
            ));
        }
        expressions::prepare_expr_common(self, simple_expr, sql)
    }

    fn prepare_select_lock(
        &self,
        _select_lock: &LockClause,
        _sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        // SQLite doesn't supports row locking
        Ok(())
    }

    fn prepare_select_into(
        &self,
        _: &SelectInto,
        _: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        Err(RenderError::Unsupported(
            "SQLite has no SELECT INTO a table".into(),
        ))
    }

    fn prepare_on_conflict_target(
        &self,
        targets: &OnConflictTarget,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        if let OnConflictTarget::Constraint(_) = targets {
            return Err(RenderError::Unsupported(
                "SQLite has no ON CONFLICT ON CONSTRAINT; name the columns".into(),
            ));
        }
        conflict::prepare_on_conflict_target(self, targets, sql)
    }

    fn prepare_sub_query_oper(
        &self,
        oper: &SubQueryOper,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        sql.write_str(match oper {
            SubQueryOper::Exists => "EXISTS",
            SubQueryOper::Any => {
                return Err(RenderError::Unsupported(
                    "SQLite has no ANY subquery operator".into(),
                ));
            }
            SubQueryOper::Some => {
                return Err(RenderError::Unsupported(
                    "SQLite has no SOME subquery operator".into(),
                ));
            }
            SubQueryOper::All => {
                return Err(RenderError::Unsupported(
                    "SQLite has no ALL subquery operator".into(),
                ));
            }
        })
        .unwrap();
        Ok(())
    }

    fn prepare_bin_oper(
        &self,
        bin_oper: &BinOper,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match bin_oper {
            BinOper::SqliteOperator(oper) => sql.write_str(oper.sql()).unwrap(),
            _ => expressions::prepare_bin_oper_common(self, bin_oper, sql)?,
        }
        Ok(())
    }

    fn prepare_union_statement(
        &self,
        union_type: UnionType,
        select_statement: &SelectStatement,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match union_type {
            UnionType::Intersect => sql.write_str(" INTERSECT ").unwrap(),
            UnionType::Distinct => sql.write_str(" UNION ").unwrap(),
            UnionType::Except => sql.write_str(" EXCEPT ").unwrap(),
            UnionType::All => sql.write_str(" UNION ALL ").unwrap(),
        }
        // An operand is a bare select core here, so one with an ordering,
        // paging, operands or CTEs of its own is read through a derived table.
        if select_statement.orders.is_empty()
            && select_statement.limit.is_none()
            && select_statement.offset.is_none()
            && select_statement.unions.is_empty()
            && select_statement.with.is_none()
        {
            return statements::prepare_select_statement(self, select_statement, sql);
        }
        let mut derived = SelectStatement::new();
        derived
            .column(Asterisk)
            .from_subquery(select_statement.clone(), "tideorm_union_operand");
        statements::prepare_select_statement(self, &derived, sql)
    }

    fn prepare_select_limit_offset(
        &self,
        select: &SelectStatement,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        prepare_limit_offset(self, select, SqlDialect::open_limit(self), sql)
    }

    fn greatest_function(&self) -> &str {
        "MAX"
    }

    fn least_function(&self) -> &str {
        "MIN"
    }

    fn char_length_function(&self) -> &str {
        "LENGTH"
    }

    fn insert_default_values(
        &self,
        num_rows: usize,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        // `DEFAULT VALUES` inserts one row. More name the rowid, which SQLite
        // numbers when it is NULL, so every declared column takes its default;
        // a `WITHOUT ROWID` table has no rowid to name.
        if num_rows <= 1 {
            sql.write_str("DEFAULT VALUES").unwrap();
            return Ok(());
        }
        sql.write_str("(rowid) VALUES (NULL)").unwrap();
        for _ in 1..num_rows {
            sql.write_str(", (NULL)").unwrap();
        }
        Ok(())
    }
}
