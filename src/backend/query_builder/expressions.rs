use super::*;

pub(super) fn prepare_expr<B: QueryBuilder>(
    builder: &B,
    simple_expr: &Expr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_expr_common(builder, simple_expr, sql)
}

pub(crate) fn prepare_expr_common<B: QueryBuilder>(
    builder: &B,
    simple_expr: &Expr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match simple_expr {
        Expr::Column(column_ref) => {
            prepare_column_ref(builder, column_ref, sql)?;
        }
        Expr::Tuple(exprs) => {
            values::prepare_tuple(builder, exprs, sql)?;
        }
        Expr::Unary(op, expr) => {
            prepare_un_oper(builder, op, sql);
            sql.write_str(" ").unwrap();
            let drop_expr_paren =
                builder.inner_expr_well_known_greater_precedence(expr, &(*op).into());
            if !drop_expr_paren {
                sql.write_str("(").unwrap();
            }
            builder.prepare_expr(expr, sql)?;
            if !drop_expr_paren {
                sql.write_str(")").unwrap();
            }
        }
        Expr::FunctionCall(func) => match (&func.func, func.args.as_slice()) {
            // One value is its own greatest and least: SQLite's one-argument
            // MAX and MIN are aggregates, and MySQL's GREATEST and LEAST take
            // two or more.
            (Func::Greatest | Func::Least, [arg]) => {
                sql.write_str("(").unwrap();
                builder.prepare_expr(arg, sql)?;
                sql.write_str(")").unwrap();
            }
            (Func::Greatest | Func::Least, []) => {
                return Err(RenderError::Unsupported(
                    "GREATEST and LEAST need at least one argument".into(),
                ));
            }
            _ => {
                builder.prepare_function_name(&func.func, sql)?;
                functions::prepare_function_arguments(builder, func, sql)?;
            }
        },
        Expr::Binary(left, op, right) => match (op, right.as_ref()) {
            (BinOper::In, Expr::Tuple(t)) if t.is_empty() => sql.write_str("1 = 2").unwrap(),
            (BinOper::NotIn, Expr::Tuple(t)) if t.is_empty() => sql.write_str("1 = 1").unwrap(),
            _ => conflict::binary_expr(builder, left, op, right, sql)?,
        },
        Expr::SubQuery(oper, sel) => {
            if let Some(oper) = oper {
                builder.prepare_sub_query_oper(oper, sql)?;
            }
            sql.write_str("(").unwrap();
            builder.prepare_query_statement(sel.deref(), sql)?;
            sql.write_str(")").unwrap();
        }
        Expr::Value(val) => {
            builder.prepare_value(val.clone(), sql)?;
        }
        Expr::Values(list) => {
            sql.write_str("(").unwrap();
            let mut iter = list.iter();
            join_io!(
                iter,
                val,
                join {
                    sql.write_str(", ").unwrap();
                },
                do {
                    builder.prepare_value(val.clone(), sql)?;
                }
            );
            sql.write_str(")").unwrap();
        }
        Expr::Custom(s) => {
            sql.write_str(s).unwrap();
        }
        Expr::CustomWithExpr(expr) => {
            for part in &expr.parts {
                match part {
                    crate::expr::CustomExprPart::Text(text) => sql.write_str(text).unwrap(),
                    crate::expr::CustomExprPart::Expression(expr) => {
                        builder.prepare_expr(expr, sql)?
                    }
                }
            }
        }
        Expr::Invalid(error) => return Err(error.clone()),
        Expr::Keyword(keyword) => {
            values::prepare_keyword(builder, keyword, sql)?;
        }
        Expr::AsEnum(_, expr) => {
            builder.prepare_expr(expr, sql)?;
        }
        Expr::Case(case_stmt) => {
            prepare_case_statement(builder, case_stmt, sql)?;
        }
        Expr::Constant(val) => {
            values::prepare_constant(builder, val, sql)?;
        }
        Expr::TypeName(type_name) => {
            functions::prepare_type_ref(builder, type_name, sql);
        }
    }
    Ok(())
}

pub(super) fn prepare_case_statement<B: QueryBuilder>(
    builder: &B,
    stmts: &CaseStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str("(CASE").unwrap();

    let CaseStatement { when, r#else } = stmts;

    for case in when.iter() {
        sql.write_str(" WHEN (").unwrap();
        conflict::prepare_condition_where(builder, &case.condition, sql)?;
        sql.write_str(") THEN ").unwrap();

        builder.prepare_expr(&case.result, sql)?;
    }
    if let Some(r#else) = r#else {
        sql.write_str(" ELSE ").unwrap();
        builder.prepare_expr(r#else, sql)?;
    }

    sql.write_str(" END)").unwrap();
    Ok(())
}

pub(crate) fn prepare_select_distinct<B: QueryBuilder>(
    _builder: &B,
    select_distinct: &SelectDistinct,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match select_distinct {
        SelectDistinct::All => sql.write_str("ALL").unwrap(),
        // MySQL's DISTINCTROW is a synonym of DISTINCT.
        SelectDistinct::Distinct | SelectDistinct::DistinctRow => {
            sql.write_str("DISTINCT").unwrap()
        }
        SelectDistinct::DistinctOn(_) => {
            return Err(RenderError::Unsupported(
                "DISTINCT ON is PostgreSQL only".into(),
            ));
        }
    }
    Ok(())
}

pub(crate) fn prepare_select_lock<B: QueryBuilder>(
    builder: &B,
    lock: &LockClause,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str(" ").unwrap();
    sql.write_str(builder.lock_phrase(lock.r#type)).unwrap();
    let mut tables = lock.tables.iter();
    join_io!(
        tables,
        table_ref,
        first {
            sql.write_str(" OF ").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            prepare_table_ref(builder, table_ref, sql)?;
        }
    );

    if let Some(behavior) = lock.behavior {
        match behavior {
            LockBehavior::Nowait => sql.write_str(" NOWAIT").unwrap(),
            LockBehavior::SkipLocked => sql.write_str(" SKIP LOCKED").unwrap(),
        }
    }
    Ok(())
}

pub(super) fn prepare_select_expr<B: QueryBuilder>(
    builder: &B,
    select_expr: &SelectExpr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    builder.prepare_expr(&select_expr.expr, sql)?;
    match &select_expr.window {
        Some(WindowSelectType::Name(name)) => {
            sql.write_str(" OVER ").unwrap();
            builder.prepare_iden(name, sql);
        }
        Some(WindowSelectType::Query(window)) => {
            sql.write_str(" OVER ").unwrap();
            sql.write_str("( ").unwrap();
            conflict::prepare_window_statement(builder, window, sql)?;
            sql.write_str(" )").unwrap();
        }
        None => {}
    };

    if let Some(alias) = &select_expr.alias {
        sql.write_str(" AS ").unwrap();
        builder.prepare_iden(alias, sql);
    };
    Ok(())
}

pub(super) fn prepare_join_expr<B: QueryBuilder>(
    builder: &B,
    join_expr: &JoinExpr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    builder.prepare_join_type(&join_expr.join, sql)?;
    sql.write_str(" ").unwrap();
    prepare_join_table_ref(builder, join_expr, sql)?;
    if let Some(on) = &join_expr.on {
        functions::prepare_join_on(builder, on, sql)?;
    }
    Ok(())
}

pub(super) fn prepare_join_table_ref<B: QueryBuilder>(
    builder: &B,
    join_expr: &JoinExpr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if join_expr.lateral {
        sql.write_str("LATERAL ").unwrap();
    }
    prepare_table_ref(builder, &join_expr.table, sql)?;
    Ok(())
}

pub(crate) fn prepare_table_ref<B: QueryBuilder>(
    builder: &B,
    table_ref: &TableRef,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match table_ref {
        TableRef::SubQuery(query, alias) => {
            sql.write_str("(").unwrap();
            statements::prepare_select_statement(builder, query, sql)?;
            sql.write_str(")").unwrap();
            sql.write_str(" AS ").unwrap();
            builder.prepare_iden(alias, sql);
        }
        TableRef::ValuesList(values, alias) => {
            sql.write_str("(").unwrap();
            values::prepare_values_list(builder, values, sql)?;
            sql.write_str(")").unwrap();
            sql.write_str(" AS ").unwrap();
            builder.prepare_iden(alias, sql);
        }
        TableRef::FunctionCall(func, alias) => {
            builder.prepare_function_name(&func.func, sql)?;
            functions::prepare_function_arguments(builder, func, sql)?;
            sql.write_str(" AS ").unwrap();
            builder.prepare_iden(alias, sql);
        }
        _ => builder.prepare_table_ref_iden(table_ref, sql)?,
    }
    Ok(())
}

pub(crate) fn prepare_column_ref<B: QueryBuilder>(
    builder: &B,
    column_ref: &ColumnRef,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match column_ref {
        ColumnRef::Column(ColumnName(table_name, column)) => {
            if let Some(table_name) = table_name {
                builder.prepare_table_name(table_name, sql);
                sql.write_str(".").unwrap();
            }
            builder.prepare_iden(column, sql);
        }
        ColumnRef::Asterisk(table_name) => {
            if let Some(table_name) = table_name {
                builder.prepare_table_name(table_name, sql);
                sql.write_str(".").unwrap();
            }
            sql.write_str("*").unwrap();
        }
    }
    Ok(())
}

pub(super) fn prepare_un_oper<B: QueryBuilder>(
    _builder: &B,
    un_oper: &UnOper,
    sql: &mut impl SqlWriter,
) {
    sql.write_str(match un_oper {
        UnOper::Not => "NOT",
    })
    .unwrap();
}

pub(crate) fn prepare_bin_oper_common<B: QueryBuilder>(
    _builder: &B,
    bin_oper: &BinOper,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_str(match bin_oper {
        BinOper::And => "AND",
        BinOper::Or => "OR",
        BinOper::Like => "LIKE",
        BinOper::NotLike => "NOT LIKE",
        BinOper::Is => "IS",
        BinOper::IsNot => "IS NOT",
        BinOper::In => "IN",
        BinOper::NotIn => "NOT IN",
        BinOper::Between => "BETWEEN",
        BinOper::NotBetween => "NOT BETWEEN",
        BinOper::Equal => "=",
        BinOper::NotEqual => "<>",
        BinOper::SmallerThan => "<",
        BinOper::GreaterThan => ">",
        BinOper::SmallerThanOrEqual => "<=",
        BinOper::GreaterThanOrEqual => ">=",
        BinOper::Add => "+",
        BinOper::Sub => "-",
        BinOper::Mul => "*",
        BinOper::Div => "/",
        BinOper::Mod => "%",
        BinOper::LShift => "<<",
        BinOper::RShift => ">>",
        BinOper::As => "AS",
        BinOper::Escape => "ESCAPE",
        BinOper::Custom(raw) => raw,
        BinOper::BitAnd => "&",
        BinOper::BitOr => "|",
        #[allow(unreachable_patterns)]
        _ => {
            return Err(RenderError::Unsupported(format!(
                "{bin_oper:?} needs a backend-specific builder"
            )));
        }
    })
    .unwrap();
    Ok(())
}

pub(super) fn prepare_bin_oper<B: QueryBuilder>(
    builder: &B,
    bin_oper: &BinOper,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_bin_oper_common(builder, bin_oper, sql)
}

pub(super) fn prepare_sub_query_oper<B: QueryBuilder>(
    _builder: &B,
    oper: &SubQueryOper,
    sql: &mut impl SqlWriter,
) {
    sql.write_str(match oper {
        SubQueryOper::Exists => "EXISTS",
        SubQueryOper::Any => "ANY",
        SubQueryOper::Some => "SOME",
        SubQueryOper::All => "ALL",
    })
    .unwrap();
}

pub(super) fn prepare_logical_chain_oper<B: QueryBuilder>(
    builder: &B,
    log_chain_oper: &LogicalChainOper,
    i: usize,
    length: usize,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let (simple_expr, oper) = match log_chain_oper {
        LogicalChainOper::And(simple_expr) => (simple_expr, "AND"),
        LogicalChainOper::Or(simple_expr) => (simple_expr, "OR"),
    };
    if i > 0 {
        sql.write_str(" ").unwrap();
        sql.write_str(oper).unwrap();
        sql.write_str(" ").unwrap();
    }
    let both_binary = match simple_expr {
        Expr::Binary(_, _, right) => {
            matches!(right.as_ref(), Expr::Binary(_, _, _))
        }
        _ => false,
    };
    let need_parentheses = length > 1 && both_binary;
    if need_parentheses {
        sql.write_str("(").unwrap();
    }
    builder.prepare_expr(simple_expr, sql)?;
    if need_parentheses {
        sql.write_str(")").unwrap();
    }
    Ok(())
}
