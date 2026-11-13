use super::*;

pub(super) fn prepare_on_conflict<B: QueryBuilder>(
    builder: &B,
    on_conflict: &Option<OnConflict>,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(on_conflict) = on_conflict {
        builder.prepare_on_conflict_keywords(sql)?;
        builder.prepare_on_conflict_target(&on_conflict.targets, sql)?;
        builder.prepare_on_conflict_condition(&on_conflict.target_where, sql)?;
        builder.prepare_on_conflict_action(&on_conflict.action, sql)?;
        builder.prepare_on_conflict_action_condition(&on_conflict.action_where, sql)?;
    }
    Ok(())
}

pub(crate) fn prepare_on_conflict_target<B: QueryBuilder>(
    builder: &B,
    on_conflict_targets: &OnConflictTarget,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match on_conflict_targets {
        OnConflictTarget::Identifiers(identifiers) => {
            prepare_on_conflict_target_identifiers(builder, identifiers, sql)?
        }
        OnConflictTarget::Constraint(constraint) => {
            prepare_on_conflict_target_constraint(builder, constraint, sql)
        }
    }
    Ok(())
}

pub(super) fn prepare_on_conflict_target_identifiers<B: QueryBuilder>(
    builder: &B,
    identifiers: &[OnConflictIdentifier],
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let mut targets = identifiers.iter();
    join_io!(
        targets,
        target,
        first {
            sql.write_str("(").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            match target {
                OnConflictIdentifier::Column(col) => builder.prepare_iden(col, sql),
                OnConflictIdentifier::Expr(expr) => {
                    // https://www.postgresql.org/docs/current/sql-insert.html
                    let need_wrap = !matches!(
                        expr,
                        Expr::Column(_)
                            | Expr::Tuple(_)
                            | Expr::FunctionCall(_)
                            | Expr::Value(_)
                            | Expr::Values(_)
                            | Expr::Constant(_)
                    );
                    if need_wrap {
                        sql.write_str("(").unwrap();
                    }
                    builder.prepare_expr(expr, sql)?;
                    if need_wrap {
                        sql.write_str(")").unwrap();
                    }
                }
            }
        },
        last {
            sql.write_str(")").unwrap();
        }
    );
    Ok(())
}

pub(super) fn prepare_on_conflict_target_constraint<B: QueryBuilder>(
    builder: &B,
    constraint: &str,
    sql: &mut impl SqlWriter,
) {
    sql.write_str("ON CONSTRAINT ").unwrap();
    crate::backend::write_identifier(sql, builder.quote(), constraint);
}

pub(super) fn prepare_on_conflict_action<B: QueryBuilder>(
    builder: &B,
    on_conflict_action: &Option<OnConflictAction>,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_on_conflict_action_common(builder, on_conflict_action, sql)
}

pub(crate) fn prepare_on_conflict_action_common<B: QueryBuilder>(
    builder: &B,
    on_conflict_action: &Option<OnConflictAction>,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(action) = on_conflict_action {
        match action {
            OnConflictAction::DoNothing(_) => {
                sql.write_str(" DO NOTHING").unwrap();
            }
            OnConflictAction::Update(update_strats) => {
                builder.prepare_on_conflict_do_update_keywords(sql)?;
                let mut update_strats_iter = update_strats.iter();
                join_io!(
                    update_strats_iter,
                    update_strat,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        match update_strat {
                            OnConflictUpdate::Column(col) => {
                                builder.prepare_iden(col, sql);
                                sql.write_str(" = ").unwrap();
                                builder.prepare_on_conflict_excluded_table(col, sql)?;
                            }
                            OnConflictUpdate::Expr(col, expr) => {
                                builder.prepare_iden(col, sql);
                                sql.write_str(" = ").unwrap();
                                builder.prepare_expr(expr, sql)?;
                            }
                        }
                    }
                );
            }
        }
    }
    Ok(())
}

pub(super) fn prepare_on_conflict_keywords<B: QueryBuilder>(
    _builder: &B,
    sql: &mut impl SqlWriter,
) {
    sql.write_str(" ON CONFLICT ").unwrap();
}

pub(super) fn prepare_on_conflict_do_update_keywords<B: QueryBuilder>(
    _builder: &B,
    sql: &mut impl SqlWriter,
) {
    sql.write_str(" DO UPDATE SET ").unwrap();
}

pub(super) fn prepare_on_conflict_excluded_table<B: QueryBuilder>(
    builder: &B,
    col: &DynIden,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    sql.write_char(builder.quote().left()).unwrap();
    sql.write_str("excluded").unwrap();
    sql.write_char(builder.quote().right()).unwrap();
    sql.write_str(".").unwrap();
    builder.prepare_iden(col, sql);
    Ok(())
}

pub(super) fn prepare_on_conflict_condition<B: QueryBuilder>(
    builder: &B,
    on_conflict_condition: &ConditionHolder,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    prepare_condition(builder, on_conflict_condition, "WHERE", sql)
}

pub(super) fn prepare_returning<B: QueryBuilder>(
    builder: &B,
    returning: &Option<ReturningClause>,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    if let Some(returning) = returning {
        sql.write_str(" RETURNING ").unwrap();
        match &returning {
            ReturningClause::All => sql.write_str("*").unwrap(),
            ReturningClause::Columns(cols) => {
                let mut cols_iter = cols.iter();
                join_io!(
                    cols_iter,
                    column_ref,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        expressions::prepare_column_ref(builder, column_ref, sql)?;
                    }
                );
            }
            ReturningClause::Exprs(exprs) => {
                let mut exprs_iter = exprs.iter();
                join_io!(
                    exprs_iter,
                    expr,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        builder.prepare_expr(expr, sql)?;
                    }
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn prepare_condition<B: QueryBuilder>(
    builder: &B,
    condition: &ConditionHolder,
    keyword: &str,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    match &condition.contents {
        ConditionHolderContents::Empty => (),
        ConditionHolderContents::Chain(conditions) => {
            sql.write_str(" ").unwrap();
            sql.write_str(keyword).unwrap();
            sql.write_str(" ").unwrap();
            for (i, log_chain_oper) in conditions.iter().enumerate() {
                expressions::prepare_logical_chain_oper(
                    builder,
                    log_chain_oper,
                    i,
                    conditions.len(),
                    sql,
                )?;
            }
        }
        ConditionHolderContents::Condition(c) => {
            sql.write_str(" ").unwrap();
            sql.write_str(keyword).unwrap();
            sql.write_str(" ").unwrap();
            prepare_condition_where(builder, c, sql)?;
        }
    }
    Ok(())
}

pub(super) fn prepare_condition_where<B: QueryBuilder>(
    builder: &B,
    condition: &Condition,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let simple_expr = condition.clone().into();
    builder.prepare_expr(&simple_expr, sql)?;
    Ok(())
}

/// One bound of a window frame, an offset written as an integer literal,
/// which every backend takes there.
pub(super) fn prepare_frame(frame: &Frame, sql: &mut impl SqlWriter) {
    match *frame {
        Frame::UnboundedPreceding => sql.write_str("UNBOUNDED PRECEDING"),
        Frame::Preceding(v) => write!(sql, "{v} PRECEDING"),
        Frame::CurrentRow => sql.write_str("CURRENT ROW"),
        Frame::Following(v) => write!(sql, "{v} FOLLOWING"),
        Frame::UnboundedFollowing => sql.write_str("UNBOUNDED FOLLOWING"),
    }
    .unwrap();
}

pub(super) fn prepare_window_statement<B: QueryBuilder>(
    builder: &B,
    window: &WindowStatement,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    let mut partition_iter = window.partition_by.iter();
    join_io!(
        partition_iter,
        expr,
        first {
            sql.write_str("PARTITION BY ").unwrap();
        },
        join {
            sql.write_str(", ").unwrap();
        },
        do {
            builder.prepare_expr(expr, sql)?;
        }
    );

    statements::prepare_order_by(builder, &window.order_by, sql)?;

    if let Some(frame) = &window.frame {
        match frame.r#type {
            FrameType::Range => sql.write_str(" RANGE ").unwrap(),
            FrameType::Rows => sql.write_str(" ROWS ").unwrap(),
        };
        if let Some(end) = &frame.end {
            sql.write_str("BETWEEN ").unwrap();
            prepare_frame(&frame.start, sql);
            sql.write_str(" AND ").unwrap();
            prepare_frame(end, sql);
        } else {
            prepare_frame(&frame.start, sql);
        }
    }
    Ok(())
}

pub(super) fn binary_expr<B: QueryBuilder>(
    builder: &B,
    left: &Expr,
    op: &BinOper,
    right: &Expr,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    // If left has higher precedence than op, we can drop parentheses around left
    let drop_left_higher_precedence =
        builder.inner_expr_well_known_greater_precedence(left, &(*op).into());

    // Figure out if left associativity rules allow us to drop the left parenthesis
    let drop_left_assoc = left.is_binary()
        && op == left.get_bin_oper().unwrap()
        && builder.well_known_left_associative(op);

    let left_paren = !drop_left_higher_precedence && !drop_left_assoc;
    if left_paren {
        sql.write_str("(").unwrap();
    }
    builder.prepare_expr(left, sql)?;
    if left_paren {
        sql.write_str(")").unwrap();
    }

    sql.write_str(" ").unwrap();
    builder.prepare_bin_oper(op, sql)?;
    sql.write_str(" ").unwrap();

    // If right has higher precedence than op, we can drop parentheses around right
    let drop_right_higher_precedence =
        builder.inner_expr_well_known_greater_precedence(right, &(*op).into());

    let op_as_oper = Oper::BinOper(*op);
    // Due to representation of trinary op between as nested binary ops.
    let drop_right_between_hack = op_as_oper.is_between()
        && right.is_binary()
        && matches!(right.get_bin_oper(), Some(&BinOper::And));

    // Due to representation of trinary op like/not like with optional arg escape as nested binary ops.
    let drop_right_escape_hack = op_as_oper.is_like()
        && right.is_binary()
        && matches!(right.get_bin_oper(), Some(&BinOper::Escape));

    // Due to custom representation of casting AS datatype
    let drop_right_as_hack = (op == &BinOper::As) && matches!(right, Expr::Custom(_));

    let right_paren = !drop_right_higher_precedence
        && !drop_right_escape_hack
        && !drop_right_between_hack
        && !drop_right_as_hack;
    if right_paren {
        sql.write_str("(").unwrap();
    }
    builder.prepare_expr(right, sql)?;
    if right_paren {
        sql.write_str(")").unwrap();
    }
    Ok(())
}
