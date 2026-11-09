use super::*;

impl SubQueryStatement {
    pub(crate) fn prepare_statement(
        &self,
        query_builder: &impl QueryBuilder,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        use SubQueryStatement::*;
        match self {
            SelectStatement(stmt) => {
                statements::prepare_select_statement(query_builder, stmt, sql)?
            }
            InsertStatement(stmt) => {
                statements::prepare_insert_statement(query_builder, stmt, sql)?
            }
            UpdateStatement(stmt) => {
                statements::prepare_update_statement(query_builder, stmt, sql)?
            }
            DeleteStatement(stmt) => {
                statements::prepare_delete_statement(query_builder, stmt, sql)?
            }
            WithStatement(stmt) => functions::prepare_with_query(query_builder, stmt, sql)?,
        }
        Ok(())
    }
}

pub(crate) struct CommonSqlQueryBuilder;

impl OperLeftAssocDecider for CommonSqlQueryBuilder {}

impl PrecedenceDecider for CommonSqlQueryBuilder {}

impl QueryBuilder for CommonSqlQueryBuilder {
    fn lexer_dialect(&self) -> crate::lexer::Dialect {
        crate::lexer::Dialect::Any
    }

    fn prepare_select_into(
        &self,
        _: &SelectInto,
        _: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        Ok(())
    }
}

impl QuotedBuilder for CommonSqlQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for CommonSqlQueryBuilder {}

impl TableRefBuilder for CommonSqlQueryBuilder {}

pub(crate) fn common_inner_expr_well_known_greater_precedence(
    inner: &Expr,
    outer_oper: &Oper,
) -> bool {
    match inner {
        // We only consider the case where an inner expression is contained in either a
        // unary or binary expression (with an outer_oper).
        // We do not need to wrap with parentheses:
        // Columns, tuples (already wrapped), constants, function calls, values,
        // keywords, subqueries (already wrapped), case (already wrapped)
        Expr::Column(_)
        | Expr::Tuple(_)
        | Expr::Constant(_)
        | Expr::FunctionCall(_)
        | Expr::Value(_)
        | Expr::Keyword(_)
        | Expr::Case(_)
        | Expr::SubQuery(_, _)
        | Expr::TypeName(_) => true,
        Expr::Binary(_, inner_oper, _) => {
            let inner_oper: Oper = (*inner_oper).into();
            if inner_oper.is_arithmetic() || inner_oper.is_shift() {
                outer_oper.is_comparison()
                    || outer_oper.is_between()
                    || outer_oper.is_in()
                    || outer_oper.is_like()
                    || outer_oper.is_logical()
            } else if inner_oper.is_comparison()
                || inner_oper.is_in()
                || inner_oper.is_like()
                || inner_oper.is_is()
            {
                outer_oper.is_logical()
            } else {
                false
            }
        }
        _ => false,
    }
}

/// A select's ` LIMIT n OFFSET ?`: the limit written in, `open_limit` for an
/// offset alone where the backend has no bare `OFFSET`, and the offset bound.
pub(crate) fn prepare_limit_offset(
    builder: &impl QueryBuilder,
    select: &SelectStatement,
    open_limit: Option<&str>,
    sql: &mut impl SqlWriter,
) -> Result<(), RenderError> {
    // The limit is written in, not bound: SQLite 3.50 and later recompile a
    // statement whose LIMIT is a parameter every time it runs. It is a `u64`.
    match (&select.limit, open_limit) {
        (Some(limit), _) => {
            sql.write_str(" LIMIT ").unwrap();
            builder.write_value(sql, limit)?;
        }
        (None, Some(open)) if select.offset.is_some() => {
            sql.write_str(" LIMIT ").unwrap();
            sql.write_str(open).unwrap();
        }
        _ => {}
    }

    if let Some(offset) = &select.offset {
        sql.write_str(" OFFSET ").unwrap();
        builder.prepare_value(offset.clone(), sql)?;
    }
    Ok(())
}

pub(crate) fn common_well_known_left_associative(op: &BinOper) -> bool {
    matches!(
        op,
        BinOper::And | BinOper::Or | BinOper::Add | BinOper::Sub | BinOper::Mul | BinOper::Mod
    )
}

macro_rules! join_io {
    ($iter:ident, $item:ident $(, first $first:expr)?, join $join:expr, do $do:expr $(, last $last:expr)?) => {
        if let Some($item) = $iter.next() {
            $($first)?
            $do

            for $item in $iter {
                $join
                $do
            }

            $($last)?
        }
    };
}

pub(crate) use join_io;
