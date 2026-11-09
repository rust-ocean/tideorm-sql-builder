use super::*;

pub(super) fn placeholder<B: QueryBuilder>(_builder: &B) -> (&'static str, bool) {
    ("?", false)
}

pub(super) fn values_list_tuple_prefix<B: QueryBuilder>(_builder: &B) -> &str {
    ""
}

pub(crate) fn prepare_insert_statement<B: QueryBuilder>(
    builder: &B,
    insert: &InsertStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(with) = &insert.with {
        functions::prepare_with_clause(builder, with, sql)?;
    }

    builder.prepare_insert(insert.replace, sql)?;

    if let Some(table) = &insert.table {
        sql.write_str(" INTO ").unwrap();

        expressions::prepare_table_ref(builder, table, sql)?;
    }

    if let Some(num_rows) = insert.active_default_rows() {
        sql.write_str(" ").unwrap();
        builder.insert_default_values(num_rows, sql)?;
    } else {
        sql.write_str(" (").unwrap();
        let mut cols = insert.columns.iter();
        join_io!(
            cols,
            col,
            join {
                sql.write_str(", ").unwrap();
            },
            do {
                builder.prepare_iden(col, sql);
            }
        );

        sql.write_str(")").unwrap();

        if let Some(source) = &insert.source {
            sql.write_str(" ").unwrap();
            match source {
                InsertValueSource::Values(values) => {
                    sql.write_str("VALUES ").unwrap();
                    let mut vals = values.iter();
                    join_io!(
                        vals,
                        row,
                        join {
                            sql.write_str(", ").unwrap();
                        },
                        do {
                            sql.write_str("(").unwrap();
                            let mut cols = row.iter();
                            join_io!(
                                cols,
                                col,
                                join {
                                    sql.write_str(", ").unwrap();
                                },
                                do {
                                    builder.prepare_expr(col, sql)?;
                                }
                            );

                            sql.write_str(")").unwrap();
                        }
                    );
                }
                InsertValueSource::Select(select_query) => {
                    prepare_select_statement(builder, select_query.deref(), sql)?;
                }
            }
        }
    }

    conflict::prepare_on_conflict(builder, &insert.on_conflict, sql)?;

    conflict::prepare_returning(builder, &insert.returning, sql)?;
    Ok(())
}

pub(super) fn prepare_union_statement<B: QueryBuilder>(
    builder: &B,
    union_type: UnionType,
    select_statement: &SelectStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match union_type {
        UnionType::Intersect => sql.write_str(" INTERSECT (").unwrap(),
        UnionType::Distinct => sql.write_str(" UNION (").unwrap(),
        UnionType::Except => sql.write_str(" EXCEPT (").unwrap(),
        UnionType::All => sql.write_str(" UNION ALL (").unwrap(),
    }
    prepare_select_statement(builder, select_statement, sql)?;
    sql.write_str(")").unwrap();
    Ok(())
}

pub(crate) fn prepare_select_statement<B: QueryBuilder>(
    builder: &B,
    select: &SelectStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(custom) = &select.custom {
        return builder.prepare_expr(&custom.clone().into(), sql);
    }

    if let Some(with) = &select.with {
        functions::prepare_with_clause(builder, with, sql)?;
    }

    sql.write_str("SELECT ").unwrap();

    if let Some(distinct) = &select.distinct {
        builder.prepare_select_distinct(distinct, sql)?;
        sql.write_str(" ").unwrap();
    }

    let mut selects = select.selects.iter();
    join_io!(
        selects,
        expr,
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            expressions::prepare_select_expr(builder, expr, sql)?;
        }
    );

    if let Some(into_table) = &select.into {
        builder.prepare_select_into(into_table, sql)?;
    }

    let mut from_tables = select.from.iter();
    join_io!(
        from_tables,
        table_ref,
        first {
            sql.write_str(" FROM ").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            expressions::prepare_table_ref(builder, table_ref, sql)?;
        }
    );

    for expr in &select.join {
        sql.write_str(" ").unwrap();
        expressions::prepare_join_expr(builder, expr, sql)?;
    }

    conflict::prepare_condition(builder, &select.r#where, "WHERE", sql)?;

    let mut groups = select.groups.iter();
    join_io!(
        groups,
        expr,
        first {
            sql.write_str(" GROUP BY ").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            builder.prepare_expr(expr, sql)?;
        }
    );

    conflict::prepare_condition(builder, &select.having, "HAVING", sql)?;

    if let Some((name, query)) = &select.window {
        sql.write_str(" WINDOW ").unwrap();
        builder.prepare_iden(name, sql);
        sql.write_str(" AS (").unwrap();
        conflict::prepare_window_statement(builder, query, sql)?;
        sql.write_str(")").unwrap();
    }

    if !select.unions.is_empty() {
        for (union_type, query) in &select.unions {
            builder.prepare_union_statement(*union_type, query, sql)?;
        }
    }

    prepare_order_by(builder, &select.orders, sql)?;

    builder.prepare_select_limit_offset(select, sql)?;

    if let Some(lock) = &select.lock {
        builder.prepare_select_lock(lock, sql)?;
    }
    Ok(())
}

pub(crate) fn prepare_update_statement<B: QueryBuilder>(
    builder: &B,
    update: &UpdateStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(with) = &update.with {
        functions::prepare_with_clause(builder, with, sql)?;
    }

    sql.write_str("UPDATE ").unwrap();

    if let Some(table) = &update.table {
        expressions::prepare_table_ref(builder, table, sql)?;
    }

    builder.prepare_update_join(&update.from, &update.r#where, sql)?;

    sql.write_str(" SET ").unwrap();

    let mut values = update.values.iter();
    join_io!(
        values,
        row,
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            let (col, v) = row;
            builder.prepare_update_column(&update.table, &update.from, col, sql)?;
            sql.write_str(" = ").unwrap();
            builder.prepare_expr(v, sql)?;
        }
    );

    builder.prepare_update_from(&update.from, sql)?;

    builder.prepare_update_condition(&update.from, &update.r#where, sql)?;

    builder.prepare_mutation_order_limit(&update.orders, &update.limit, sql)?;

    conflict::prepare_returning(builder, &update.returning, sql)?;
    Ok(())
}

pub(super) fn prepare_update_from<B: QueryBuilder>(
    builder: &B,
    from: &[TableRef],
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let mut from_iter = from.iter();
    join_io!(
        from_iter,
        table_ref,
        first {
            sql.write_str(" FROM ").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            expressions::prepare_table_ref(builder, table_ref, sql)?;
        }
    );
    Ok(())
}

pub(super) fn prepare_update_column<B: QueryBuilder>(
    builder: &B,
    _table: &Option<Box<TableRef>>,
    _from: &[TableRef],
    column: &DynIden,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    builder.prepare_iden(column, sql);
    Ok(())
}

pub(super) fn prepare_update_condition<B: QueryBuilder>(
    builder: &B,
    _from: &[TableRef],
    condition: &ConditionHolder,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    conflict::prepare_condition(builder, condition, "WHERE", sql)
}

pub(crate) fn prepare_delete_statement<B: QueryBuilder>(
    builder: &B,
    delete: &DeleteStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(with) = &delete.with {
        functions::prepare_with_clause(builder, with, sql)?;
    }

    sql.write_str("DELETE ").unwrap();

    if let Some(table) = &delete.table {
        sql.write_str("FROM ").unwrap();
        expressions::prepare_table_ref(builder, table, sql)?;
    }

    conflict::prepare_condition(builder, &delete.r#where, "WHERE", sql)?;

    builder.prepare_mutation_order_limit(&delete.orders, &delete.limit, sql)?;

    conflict::prepare_returning(builder, &delete.returning, sql)?;
    Ok(())
}

/// ` ORDER BY` and `orders`, when there are any.
pub(super) fn prepare_order_by<B: QueryBuilder>(
    builder: &B,
    orders: &[OrderExpr],
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let mut orders = orders.iter();
    join_io!(
        orders,
        expr,
        first {
            sql.write_str(" ORDER BY ").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            builder.prepare_order_expr(expr, sql)?;
        }
    );
    Ok(())
}

/// The ` ORDER BY` and ` LIMIT` of an `UPDATE` or `DELETE`, the limit written
/// as a value.
pub(super) fn prepare_mutation_order_limit<B: QueryBuilder>(
    builder: &B,
    orders: &[OrderExpr],
    limit: &Option<Value>,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_order_by(builder, orders, sql)?;
    if let Some(limit) = limit {
        sql.write_str(" LIMIT ").unwrap();
        builder.write_value(sql, limit)?;
    }
    Ok(())
}
