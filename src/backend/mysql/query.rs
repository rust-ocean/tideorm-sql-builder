use super::*;

/// One [`QueryBuilder`] impl for MySQL and for MariaDB, each passing the
/// methods where its server differs.
macro_rules! mysql_family_query_builder {
    ($builder:ty { $($own:tt)* }) => {
        impl QueryBuilder for $builder {
            $($own)*

            fn lexer_dialect(&self) -> crate::lexer::Dialect {
                crate::lexer::Dialect::MySql
            }

            fn prepare_select_distinct(
                &self,
                select_distinct: &SelectDistinct,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                match select_distinct {
                    SelectDistinct::All => sql.write_str("ALL").unwrap(),
                    SelectDistinct::Distinct => sql.write_str("DISTINCT").unwrap(),
                    SelectDistinct::DistinctRow => sql.write_str("DISTINCTROW").unwrap(),
                    SelectDistinct::DistinctOn(_) => {
                        return Err(RenderError::Unsupported("MySQL has no DISTINCT ON".into()));
                    }
                };
                Ok(())
            }

            fn prepare_function_filter(
                &self,
                _: &Condition,
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                Err(RenderError::Unsupported(
                    "MySQL and MariaDB have no FILTER on an aggregate; put a CASE WHEN in its argument".into(),
                ))
            }

            fn prepare_select_into(
                &self,
                _: &SelectInto,
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                Err(RenderError::Unsupported(
                    "MySQL has no SELECT INTO a table".into(),
                ))
            }

            fn prepare_with_query_clause_materialization(
                &self,
                _: &CommonTableExpression,
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                // MySQL doesn't support declaring materialization in SQL for with query.
                Ok(())
            }

            fn prepare_update_join(
                &self,
                from: &[TableRef],
                condition: &ConditionHolder,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                if from.is_empty() {
                    return Ok(());
                }

                for table in from {
                    sql.write_str(" JOIN ").unwrap();
                    expressions::prepare_table_ref(self, table, sql)?;
                }

                conflict::prepare_condition(self, condition, "ON", sql)?;
                Ok(())
            }

            fn prepare_update_from(
                &self,
                _: &[TableRef],
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                Ok(())
            }

            fn prepare_update_column(
                &self,
                table: &Option<Box<TableRef>>,
                from: &[TableRef],
                column: &DynIden,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                use std::ops::Deref;

                // A joined table may share the column's name, so the target is named:
                // by its alias when it has one, which then replaces its name.
                if !from.is_empty()
                    && let Some(table) = table
                    && let TableRef::Table(name, alias) = table.deref()
                {
                    let target = match alias {
                        Some(alias) => TableName(None, alias.clone()),
                        None => name.clone(),
                    };
                    let column_name = ColumnName(Some(target), column.clone());
                    expressions::prepare_column_ref(self, &ColumnRef::Column(column_name), sql)?;
                    return Ok(());
                }
                self.prepare_iden(column, sql);
                Ok(())
            }

            fn prepare_update_condition(
                &self,
                from: &[TableRef],
                condition: &ConditionHolder,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                if !from.is_empty() {
                    return Ok(());
                }
                conflict::prepare_condition(self, condition, "WHERE", sql)?;
                Ok(())
            }

            fn prepare_join_type(
                &self,
                join_type: &JoinType,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                match join_type {
                    JoinType::FullOuterJoin => {
                        return Err(RenderError::Unsupported(
                            "Mysql does not support FULL OUTER JOIN".into(),
                        ));
                    }
                    JoinType::StraightJoin => sql.write_str("STRAIGHT_JOIN").unwrap(),
                    _ => functions::prepare_join_type_common(self, join_type, sql)?,
                }
                Ok(())
            }

            fn prepare_order_expr(
                &self,
                order_expr: &OrderExpr,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                match order_expr.nulls {
                    None => (),
                    Some(NullOrdering::Last) => {
                        self.prepare_expr(&order_expr.expr, sql)?;
                        sql.write_str(" IS NULL ASC, ").unwrap()
                    }
                    Some(NullOrdering::First) => {
                        self.prepare_expr(&order_expr.expr, sql)?;
                        sql.write_str(" IS NULL DESC, ").unwrap()
                    }
                }
                if !matches!(order_expr.order, Order::Field(_)) {
                    self.prepare_expr(&order_expr.expr, sql)?;
                }
                functions::prepare_order(self, order_expr, sql)?;
                Ok(())
            }

            fn prepare_select_limit_offset(
                &self,
                select: &SelectStatement,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                prepare_limit_offset(self, select, SqlDialect::open_limit(self), sql)
            }

            fn prepare_on_conflict_target(
                &self,
                _: &OnConflictTarget,
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                // MySQL doesn't support declaring ON CONFLICT target.
                Ok(())
            }

            fn prepare_on_conflict_action(
                &self,
                on_conflict_action: &Option<OnConflictAction>,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                match on_conflict_action {
                    Some(OnConflictAction::DoNothing(pk_cols)) => {
                        if !pk_cols.is_empty() {
                            self.prepare_on_conflict_do_update_keywords(sql)?;
                            let mut pk_cols_iter = pk_cols.iter();
                            join_io!(
                                pk_cols_iter,
                                pk_col,
                                join {
                                    sql.write_str(", ").unwrap();
                                },
                                do {
                                    self.prepare_iden(pk_col, sql);
                                    sql.write_str(" = ").unwrap();
                                    self.prepare_iden(pk_col, sql);
                                }
                            );
                        } else {
                            return Err(RenderError::Unsupported(
                                "MySQL has no ON DUPLICATE KEY DO NOTHING; name the key columns with do_nothing_on".into(),
                            ));
                        }
                    }
                    _ => conflict::prepare_on_conflict_action_common(self, on_conflict_action, sql)?,
                }
                Ok(())
            }

            fn prepare_on_conflict_keywords(&self, sql: &mut impl SqlWriter) -> Result<(), RenderError> {
                sql.write_str(" ON DUPLICATE KEY").unwrap();
                Ok(())
            }

            fn prepare_on_conflict_do_update_keywords(
                &self,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                sql.write_str(" UPDATE ").unwrap();
                Ok(())
            }

            fn prepare_on_conflict_excluded_table(
                &self,
                col: &DynIden,
                sql: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                sql.write_str("VALUES(").unwrap();
                self.prepare_iden(col, sql);
                sql.write_str(")").unwrap();
                Ok(())
            }

            fn prepare_on_conflict_condition(
                &self,
                _: &ConditionHolder,
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                Ok(())
            }

            fn prepare_on_conflict_action_condition(
                &self,
                condition: &ConditionHolder,
                _: &mut impl SqlWriter,
            ) -> Result<(), RenderError> {
                if condition.is_empty() {
                    return Ok(());
                }
                Err(RenderError::Unsupported(
                    "MySQL has no WHERE on ON DUPLICATE KEY UPDATE".into(),
                ))
            }

            fn random_function(&self) -> &str {
                "RAND"
            }

            fn insert_default_keyword(&self) -> &str {
                "()"
            }
        }
    };
}

mysql_family_query_builder!(MysqlQueryBuilder {
    // A table value constructor's rows are `ROW(..)`, which MariaDB refuses.
    fn values_list_tuple_prefix(&self) -> &str {
        "ROW"
    }

    fn prepare_returning(
        &self,
        returning: &Option<ReturningClause>,
        _: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match returning {
            None => Ok(()),
            Some(_) => Err(RenderError::Unsupported("MySQL has no RETURNING".into())),
        }
    }

    // MariaDB orders and limits a joined UPDATE too.
    fn prepare_update_order_limit(
        &self,
        from: &[TableRef],
        orders: &[OrderExpr],
        limit: &Option<Value>,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        if !from.is_empty() && (!orders.is_empty() || limit.is_some()) {
            return Err(RenderError::Unsupported(
                "MySQL has no ORDER BY or LIMIT in an UPDATE that joins other tables".into(),
            ));
        }
        self.prepare_mutation_order_limit(orders, limit, sql)
    }

    // A shared lock is `FOR SHARE`, the form that takes `OF`, `NOWAIT` and
    // `SKIP LOCKED`; `LOCK IN SHARE MODE` takes none of them.
    fn prepare_select_lock(
        &self,
        lock: &LockClause,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        if matches!(lock.r#type, LockType::NoKeyUpdate | LockType::KeyShare) {
            return Err(RenderError::Unsupported(
                "MySQL has no FOR NO KEY UPDATE or FOR KEY SHARE lock".into(),
            ));
        }
        expressions::prepare_select_lock(self, lock, sql)
    }
});

mysql_family_query_builder!(MariaDbDialect {
    // MariaDB has no `FOR SHARE` and no `OF`: a lock covers every table the
    // SELECT reads.
    fn prepare_select_lock(
        &self,
        lock: &LockClause,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        if matches!(lock.r#type, LockType::NoKeyUpdate | LockType::KeyShare) {
            return Err(RenderError::Unsupported(
                "MariaDB has no FOR NO KEY UPDATE or FOR KEY SHARE lock".into(),
            ));
        }
        if !lock.tables.is_empty() {
            return Err(RenderError::Unsupported(
                "MariaDB has no OF in a row lock; it locks every table the SELECT reads".into(),
            ));
        }
        expressions::prepare_select_lock(self, lock, sql)
    }

    fn lock_phrase(&self, lock_type: LockType) -> &'static str {
        match lock_type {
            LockType::Update => "FOR UPDATE",
            LockType::NoKeyUpdate => "FOR NO KEY UPDATE",
            LockType::Share => "LOCK IN SHARE MODE",
            LockType::KeyShare => "FOR KEY SHARE",
        }
    }
});
