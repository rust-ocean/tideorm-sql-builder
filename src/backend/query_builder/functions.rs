use super::*;

pub(crate) fn prepare_function_name_common<B: QueryBuilder>(
    builder: &B,
    function: &Func,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Func::Custom(iden) = function {
        sql.write_str(&iden.0)
    } else {
        sql.write_str(match function {
            Func::Max => "MAX",
            Func::Min => "MIN",
            Func::Sum => "SUM",
            Func::Avg => "AVG",
            Func::Abs => "ABS",
            Func::Coalesce => "COALESCE",
            Func::Count => "COUNT",
            Func::IfNull => builder.if_null_function(),
            Func::Greatest => builder.greatest_function(),
            Func::Least => builder.least_function(),
            Func::CharLength => builder.char_length_function(),
            Func::Cast => "CAST",
            Func::Lower => "LOWER",
            Func::Upper => "UPPER",
            Func::BitAnd => "BIT_AND",
            Func::BitOr => "BIT_OR",
            Func::Custom(_) => "",
            Func::Random => builder.random_function(),
            Func::Round => "ROUND",
            Func::Md5 => "MD5",

            Func::PgFunction(_) => {
                return Err(RenderError::Unsupported(
                    "PostgreSQL functions need the PostgreSQL builder".into(),
                ));
            }
        })
    }
    .unwrap();
    Ok(())
}

pub(super) fn prepare_function_arguments<B: QueryBuilder>(
    builder: &B,
    func: &FunctionCall,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str("(").unwrap();
    let mut args = func.args.iter().zip(func.mods.iter());

    if let Some((arg, modifier)) = args.next() {
        if modifier.distinct {
            sql.write_str("DISTINCT ").unwrap();
        }
        builder.prepare_expr(arg, sql)?;
    }

    for (arg, modifier) in args {
        sql.write_str(", ").unwrap();
        if modifier.distinct {
            sql.write_str("DISTINCT ").unwrap();
        }
        builder.prepare_expr(arg, sql)?;
    }

    sql.write_str(")").unwrap();

    if let Some(filter) = &func.filter {
        builder.prepare_function_filter(filter, sql)?;
    }
    Ok(())
}

pub(super) fn prepare_function_filter<B: QueryBuilder>(
    builder: &B,
    filter: &Condition,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str(" FILTER (WHERE ").unwrap();
    conflict::prepare_condition_where(builder, filter, sql)?;
    sql.write_str(")").unwrap();
    Ok(())
}

pub(crate) fn prepare_with_query<B: QueryBuilder>(
    builder: &B,
    query: &WithQuery,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_with_clause(builder, &query.with_clause, sql)?;
    if let Some(query) = &query.query {
        builder.prepare_query_statement(query, sql)?;
    } else {
        return Err(RenderError::Unsupported("WITH query has no body".into()));
    }
    Ok(())
}

pub(super) fn prepare_with_clause<B: QueryBuilder>(
    builder: &B,
    with_clause: &WithClause,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_with_clause_start(builder, with_clause, sql);
    prepare_with_clause_common_tables(builder, with_clause, sql)?;
    Ok(())
}

pub(super) fn prepare_with_clause_common_tables<B: QueryBuilder>(
    builder: &B,
    with_clause: &WithClause,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let mut cte_first = true;
    if with_clause.cte_expressions.is_empty() {
        return Err(RenderError::Unsupported(
            "WITH has no common table expression".into(),
        ));
    }

    for cte in &with_clause.cte_expressions {
        if !cte_first {
            sql.write_str(", ").unwrap();
        }
        cte_first = false;

        prepare_with_query_clause_common_table(builder, cte, sql)?;
    }
    sql.write_str(" ").unwrap();
    Ok(())
}

pub(super) fn prepare_with_query_clause_common_table<B: QueryBuilder>(
    builder: &B,
    cte: &CommonTableExpression,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(name) = &cte.table_name {
        builder.prepare_iden(name, sql);
    } else {
        return Err(RenderError::Unsupported("CTE has no name".into()));
    }

    if cte.cols.is_empty() {
        sql.write_str(" ").unwrap();
    } else {
        sql.write_str(" (").unwrap();

        let mut col_first = true;
        for col in &cte.cols {
            if !col_first {
                sql.write_str(", ").unwrap();
            }
            col_first = false;
            builder.prepare_iden(col, sql);
        }

        sql.write_str(") ").unwrap();
    }

    sql.write_str("AS ").unwrap();

    builder.prepare_with_query_clause_materialization(cte, sql)?;

    sql.write_str("(").unwrap();

    match &cte.query {
        CteQuery::SubQuery(sub_query) => builder.prepare_query_statement(sub_query, sql)?,
        CteQuery::Values(items) if items.is_empty() => {
            return Err(RenderError::Unsupported(
                "CTE has no query or values".into(),
            ));
        }
        CteQuery::Values(items) => values::prepare_values_rows(builder, items, sql)?,
    }

    sql.write_str(")").unwrap();
    Ok(())
}

pub(super) fn prepare_with_query_clause_materialization<B: QueryBuilder>(
    _builder: &B,
    cte: &CommonTableExpression,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(materialized) = cte.materialized {
        if !materialized {
            sql.write_str("NOT MATERIALIZED ")
        } else {
            sql.write_str(" MATERIALIZED ")
        }
        .unwrap()
    }
    Ok(())
}

pub(super) fn prepare_with_clause_start<B: QueryBuilder>(
    _builder: &B,
    with_clause: &WithClause,
    sql: &mut impl SqlWriter,
) {
    sql.write_str("WITH ").unwrap();

    if with_clause.recursive {
        sql.write_str("RECURSIVE ").unwrap();
    }
}

pub(super) fn prepare_insert<B: QueryBuilder>(
    _builder: &B,
    replace: bool,
    sql: &mut impl SqlWriter,
) {
    if replace {
        sql.write_str("REPLACE").unwrap();
    } else {
        sql.write_str("INSERT").unwrap();
    }
}

pub(super) fn prepare_type_ref<B: QueryBuilder>(
    builder: &B,
    type_name: &TypeRef,
    sql: &mut impl SqlWriter,
) {
    let TypeRef(schema_name, r#type) = type_name;
    if let Some(schema_name) = schema_name {
        builder.prepare_schema_name(schema_name, sql);
        write!(sql, ".").unwrap();
    }
    let mut base = r#type.0.as_ref();
    let mut array_dims = 0;
    while let Some(stripped) = base.strip_suffix("[]") {
        base = stripped;
        array_dims += 1;
    }
    if array_dims == 0 {
        builder.prepare_iden(r#type, sql);
    } else {
        let base_iden = DynIden(Cow::Owned(base.to_string()));
        builder.prepare_iden(&base_iden, sql);
        for _ in 0..array_dims {
            sql.write_str("[]").unwrap();
        }
    }
}

pub(crate) fn prepare_join_type_common<B: QueryBuilder>(
    _builder: &B,
    join_type: &JoinType,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str(match join_type {
        JoinType::Join => "JOIN",
        JoinType::CrossJoin => "CROSS JOIN",
        JoinType::InnerJoin => "INNER JOIN",
        JoinType::LeftJoin => "LEFT JOIN",
        JoinType::RightJoin => "RIGHT JOIN",
        JoinType::FullOuterJoin => "FULL OUTER JOIN",
        JoinType::StraightJoin => {
            return Err(RenderError::Unsupported(
                "STRAIGHT_JOIN is MySQL and MariaDB only".into(),
            ));
        }
    })
    .unwrap();
    Ok(())
}

pub(super) fn prepare_order_expr<B: QueryBuilder>(
    builder: &B,
    order_expr: &OrderExpr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if !matches!(order_expr.order, Order::Field(_)) {
        builder.prepare_expr(&order_expr.expr, sql)?;
    }
    prepare_order(builder, order_expr, sql)?;
    match order_expr.nulls {
        None => (),
        Some(NullOrdering::Last) => sql.write_str(" NULLS LAST").unwrap(),
        Some(NullOrdering::First) => sql.write_str(" NULLS FIRST").unwrap(),
    }
    Ok(())
}

pub(crate) fn prepare_join_on<B: QueryBuilder>(
    builder: &B,
    join_on: &JoinOn,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match join_on {
        JoinOn::Condition(c) => conflict::prepare_condition(builder, c, "ON", sql)?,
        JoinOn::Columns(_) => {
            return Err(RenderError::Unsupported(
                "a join needs an ON condition".into(),
            ));
        }
    }
    Ok(())
}

pub(crate) fn prepare_order<B: QueryBuilder>(
    builder: &B,
    order_expr: &OrderExpr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match &order_expr.order {
        Order::Asc => sql.write_str(" ASC").unwrap(),
        Order::Desc => sql.write_str(" DESC").unwrap(),
        Order::Field(values) => prepare_field_order(builder, order_expr, values, sql)?,
    }
    Ok(())
}

pub(super) fn prepare_field_order<B: QueryBuilder>(
    builder: &B,
    order_expr: &OrderExpr,
    values: &Values,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if values.0.is_empty() {
        return Err(RenderError::Unsupported(
            "Order::Field needs at least one value".into(),
        ));
    }
    sql.write_str("CASE ").unwrap();
    let mut i = 0;
    for value in &values.0 {
        sql.write_str("WHEN ").unwrap();
        builder.prepare_expr(&order_expr.expr, sql)?;
        // `= NULL` matches no row.
        if value.is_some() {
            sql.write_str("=").unwrap();
            builder.write_value(sql, value)?;
        } else {
            sql.write_str(" IS NULL").unwrap();
        }
        sql.write_str(" THEN ").unwrap();
        write_int(sql, i);
        sql.write_str(" ").unwrap();
        i += 1;
    }

    sql.write_str("ELSE ").unwrap();
    write_int(sql, i);
    sql.write_str(" END").unwrap();
    Ok(())
}
