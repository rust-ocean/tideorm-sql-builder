use super::*;
use crate::ArrayType;
use crate::extension::postgres::*;

impl OperLeftAssocDecider for PostgresQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        let common_answer = common_well_known_left_associative(op);
        let pg_specific_answer = matches!(op, BinOper::PgOperator(PgBinOper::Concatenate));
        common_answer || pg_specific_answer
    }
}

impl PrecedenceDecider for PostgresQueryBuilder {
    fn inner_expr_well_known_greater_precedence(&self, inner: &Expr, outer_oper: &Oper) -> bool {
        let common_answer = common_inner_expr_well_known_greater_precedence(inner, outer_oper);
        let pg_specific_answer = match inner {
            Expr::Binary(_, inner_bin_oper, _) => {
                let inner_oper: Oper = (*inner_bin_oper).into();
                if inner_oper.is_arithmetic() || inner_oper.is_shift() {
                    is_ilike(inner_bin_oper)
                } else if is_pg_comparison(inner_bin_oper) {
                    outer_oper.is_logical()
                } else {
                    false
                }
            }
            _ => false,
        };
        common_answer || pg_specific_answer
    }
}

impl QueryBuilder for PostgresQueryBuilder {
    fn placeholder(&self) -> (&'static str, bool) {
        ("$", true)
    }

    fn lexer_dialect(&self) -> crate::lexer::Dialect {
        crate::lexer::Dialect::Postgres
    }

    fn prepare_expr(
        &self,
        simple_expr: &Expr,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match simple_expr {
            Expr::AsEnum(type_name, expr) => {
                sql.write_str("CAST(").unwrap();
                expressions::prepare_expr_common(self, expr, sql)?;
                let type_name = &type_name.0;
                let (ty, sfx) = if let Some(base) = type_name.strip_suffix("[]") {
                    (base, "[]")
                } else {
                    (type_name.as_ref(), "")
                };
                sql.write_str(" AS ").unwrap();
                self.prepare_iden(&DynIden(Cow::Owned(ty.to_string())), sql);
                sql.write_str(sfx).unwrap();
                sql.write_char(')').unwrap();
            }
            _ => expressions::prepare_expr_common(self, simple_expr, sql)?,
        }
        Ok(())
    }

    fn write_value(&self, buf: &mut impl SqlWriter, value: &Value) -> Result<(), RenderError> {
        match value {
            Value::Enum(v) => match v {
                OptionEnum::Some(v) => {
                    self.write_string_quoted(v.value.as_ref(), buf);
                    self.write_enum_type_hint(v.type_name.as_ref(), buf);
                }
                OptionEnum::None(_) => buf.write_str("NULL").unwrap(),
            },
            Value::Array(ArrayType::Enum(type_name), values) => {
                match values {
                    None => buf.write_str("NULL").unwrap(),
                    Some(values) if values.is_empty() => buf.write_str("'{}'").unwrap(),
                    Some(values) => {
                        let elements = values
                            .iter()
                            .map(crate::value::postgres_array::enum_array_element)
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|_| {
                                RenderError::Unsupported(
                                    "Enum array contains a NULL or non-enum element".into(),
                                )
                            })?;
                        buf.write_str("ARRAY [").unwrap();
                        for (index, element) in elements.iter().enumerate() {
                            if index > 0 {
                                buf.write_str(",").unwrap();
                            }
                            self.write_string_quoted(element.value.as_ref(), buf);
                        }
                        buf.write_str("]").unwrap();
                    }
                }
                self.write_enum_type_hint(type_name.as_ref(), buf);
            }
            _ => return values::write_value_common(self, buf, value),
        }
        Ok(())
    }

    fn prepare_select_distinct(
        &self,
        select_distinct: &SelectDistinct,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match select_distinct {
            SelectDistinct::All => sql.write_str("ALL").unwrap(),
            SelectDistinct::Distinct => sql.write_str("DISTINCT").unwrap(),
            SelectDistinct::DistinctOn(cols) => {
                sql.write_str("DISTINCT ON (").unwrap();

                let mut cols = cols.iter();
                join_io!(
                    cols,
                    col,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        expressions::prepare_column_ref(self, col, sql)?;
                    }
                );

                sql.write_str(")").unwrap();
            }
            _ => {}
        };
        Ok(())
    }

    fn prepare_select_into(
        &self,
        into_table: &SelectInto,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        sql.write_str(" INTO ").unwrap();

        if let Some(modifier) = &into_table.target_table_modifier {
            sql.write_fmt(format_args!("{modifier} ")).unwrap();
        }

        sql.write_str("TABLE ").unwrap();
        self.prepare_iden(&into_table.target_table, sql);
        Ok(())
    }

    fn prepare_insert(&self, replace: bool, sql: &mut impl SqlWriter) -> Result<(), RenderError> {
        if replace {
            return Err(RenderError::Unsupported(
                "PostgreSQL has no REPLACE; use ON CONFLICT .. DO UPDATE".into(),
            ));
        }
        sql.write_str("INSERT").unwrap();
        Ok(())
    }

    fn prepare_mutation_order_limit(
        &self,
        orders: &[OrderExpr],
        limit: &Option<Value>,
        _: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        if orders.is_empty() && limit.is_none() {
            return Ok(());
        }
        Err(RenderError::Unsupported(
            "PostgreSQL has no ORDER BY or LIMIT in UPDATE or DELETE".into(),
        ))
    }

    fn prepare_bin_oper(
        &self,
        bin_oper: &BinOper,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match bin_oper {
            BinOper::PgOperator(oper) => sql.write_str(oper.sql()).unwrap(),
            _ => expressions::prepare_bin_oper_common(self, bin_oper, sql)?,
        }
        Ok(())
    }

    fn prepare_function_name(
        &self,
        function: &Func,
        sql: &mut impl SqlWriter,
    ) -> Result<(), RenderError> {
        match function {
            Func::PgFunction(function) => sql
                .write_str(match function {
                    PgFunc::ToTsquery => "TO_TSQUERY",
                    PgFunc::ToTsvector => "TO_TSVECTOR",
                    PgFunc::PhrasetoTsquery => "PHRASETO_TSQUERY",
                    PgFunc::PlaintoTsquery => "PLAINTO_TSQUERY",
                    PgFunc::WebsearchToTsquery => "WEBSEARCH_TO_TSQUERY",
                    PgFunc::TsRank => "TS_RANK",
                    PgFunc::TsRankCd => "TS_RANK_CD",
                    PgFunc::StartsWith => "STARTS_WITH",
                    PgFunc::GenRandomUUID => "GEN_RANDOM_UUID",
                    PgFunc::JsonBuildObject => "JSON_BUILD_OBJECT",
                    PgFunc::JsonAgg => "JSON_AGG",
                    PgFunc::ArrayAgg => "ARRAY_AGG",
                    PgFunc::DateTrunc => "DATE_TRUNC",
                    PgFunc::Any => "ANY",
                    PgFunc::Some => "SOME",
                    PgFunc::All => "ALL",
                    PgFunc::AdvisoryLock => "PG_ADVISORY_LOCK",
                    PgFunc::AdvisoryLockShared => "PG_ADVISORY_LOCK_SHARED",
                    PgFunc::TryAdvisoryLock => "PG_TRY_ADVISORY_LOCK",
                    PgFunc::TryAdvisoryLockShared => "PG_TRY_ADVISORY_LOCK_SHARED",
                    PgFunc::AdvisoryUnlock => "PG_ADVISORY_UNLOCK",
                    PgFunc::AdvisoryUnlockShared => "PG_ADVISORY_UNLOCK_SHARED",
                    PgFunc::AdvisoryUnlockAll => "PG_ADVISORY_UNLOCK_ALL",
                    PgFunc::AdvisoryXactLock => "PG_ADVISORY_XACT_LOCK",
                    PgFunc::AdvisoryXactLockShared => "PG_ADVISORY_XACT_LOCK_SHARED",
                    PgFunc::TryAdvisoryXactLock => "PG_TRY_ADVISORY_XACT_LOCK",
                    PgFunc::TryAdvisoryXactLockShared => "PG_TRY_ADVISORY_XACT_LOCK_SHARED",
                })
                .unwrap(),
            _ => functions::prepare_function_name_common(self, function, sql)?,
        }
        Ok(())
    }

    fn prepare_value(&self, value: Value, sql: &mut impl SqlWriter) -> Result<(), RenderError> {
        match value {
            Value::Enum(value) => match value {
                OptionEnum::Some(value) => {
                    let Enum { type_name, value } = *value;
                    sql.push_param(Value::String(Some(value.into_owned())), self as _)?;
                    self.write_enum_type_hint(type_name, sql);
                }
                OptionEnum::None(type_name) => {
                    sql.push_param(Value::String(None), self as _)?;
                    self.write_enum_type_hint(type_name, sql);
                }
            },
            Value::Array(ArrayType::Enum(type_name), values) => {
                let values = values
                    .map(|values| {
                        values
                            .iter()
                            .map(|value| {
                                crate::value::postgres_array::enum_array_element(value)
                                    .map(|value| Value::String(Some(value.value.to_string())))
                            })
                            .collect::<Result<Vec<_>, _>>()
                            .map(Box::new)
                    })
                    .transpose();
                let values = match values {
                    Ok(values) => values,
                    Err(_) => {
                        return Err(RenderError::Unsupported(
                            "Enum array contains a NULL or non-enum element".into(),
                        ));
                    }
                };
                sql.push_param(Value::Array(ArrayType::String, values), self as _)?;
                self.write_enum_type_hint(type_name.as_ref(), sql);
                sql.write_str("[]").unwrap();
            }
            _ => sql.push_param(value, self as _)?,
        }
        Ok(())
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut impl Write) {
        if self.needs_escape(string) {
            buffer.write_str("E'").unwrap();
        } else {
            buffer.write_str("'").unwrap();
        }
        self.write_escaped(buffer, string);
        buffer.write_str("'").unwrap();
    }

    fn write_bytes(&self, bytes: &[u8], buffer: &mut impl Write) {
        buffer.write_str("'\\x").unwrap();
        for b in bytes {
            write!(buffer, "{b:02X}").unwrap();
        }
        buffer.write_str("'").unwrap();
    }

    fn if_null_function(&self) -> &str {
        "COALESCE"
    }
}

impl PostgresQueryBuilder {
    fn write_enum_type_hint(&self, type_name: impl AsRef<str>, sql: &mut impl SqlWriter) {
        sql.write_str("::").unwrap();
        self.prepare_iden(&DynIden(Cow::Owned(type_name.as_ref().to_string())), sql);
    }
}

fn is_pg_comparison(b: &BinOper) -> bool {
    matches!(
        b,
        BinOper::PgOperator(PgBinOper::Contained)
            | BinOper::PgOperator(PgBinOper::Contains)
            | BinOper::PgOperator(PgBinOper::Similarity)
            | BinOper::PgOperator(PgBinOper::WordSimilarity)
            | BinOper::PgOperator(PgBinOper::StrictWordSimilarity)
            | BinOper::PgOperator(PgBinOper::Matches)
    )
}

fn is_ilike(b: &BinOper) -> bool {
    matches!(
        b,
        BinOper::PgOperator(PgBinOper::ILike) | BinOper::PgOperator(PgBinOper::NotILike)
    )
}
