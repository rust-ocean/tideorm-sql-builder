mod query;

pub use query::*;

use crate::{
    ColumnRef, DynIden, Expr, IdenList, IntoIden, QueryBuilder, QueryStatementBuilder, SelectExpr,
    SelectStatement, SqlWriter, SubQueryStatement, TableName, TableRef, Values,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CteQuery {
    SubQuery(Box<SubQueryStatement>),
    Values(Vec<Values>),
}

impl Default for CteQuery {
    fn default() -> Self {
        Self::Values(vec![])
    }
}

/// A table definition inside a WITH clause ([WithClause]).
///
/// A WITH clause can contain one or multiple common table expressions ([CommonTableExpression]).
///
/// These named queries can act as a "query local table" that are materialized during execution and
/// then can be used by the query prefixed with the WITH clause.
///
/// A WITH clause can contain multiple of these [CommonTableExpression]. (Except in the case of
/// recursive WITH query which can only contain one [CommonTableExpression]).
///
/// A [CommonTableExpression] is a name, column names and a query returning data for those columns.
///
/// Some databases (like sqlite) restrict the acceptable kinds of queries inside of the WITH clause
/// common table expressions. These databases only allow [SelectStatement]s to form a common table
/// expression.
///
/// Other databases like postgres allow modification queries (UPDATE, DELETE) inside of the WITH
/// clause but they have to return a table. (They must have a RETURNING clause).
///
/// tideorm-sql-builder doesn't check this or restrict the kind of [CommonTableExpression] that you can create
/// in rust. This means that you can put an UPDATE or DELETE queries into WITH clause and tideorm-sql-builder
/// will succeed in generating that kind of sql query but the execution inside the database will
/// fail because they are invalid.
///
/// It is your responsibility to ensure that the kind of WITH clause that you put together makes
/// sense and valid for that database that you are using.
///
/// NOTE that for recursive WITH queries (in sql: "WITH RECURSIVE") you can only have a
/// single [CommonTableExpression] inside of the WITH clause. That query must match certain
/// requirements:
///   * It is a query of UNION or UNION ALL of two queries.
///   * The first part of the query (the left side of the UNION) must be executable first in itself.
///     It must be non-recursive. (Cannot contain self reference)
///   * The self reference must appear in the right hand side of the UNION.
///   * The query can only have a single self-reference.
///   * Recursive data-modifying statements are not supported, but you can use the results of a
///     recursive SELECT query in a data-modifying statement. (like so: WITH RECURSIVE
///     cte_name(a,b,c,d) AS (SELECT ... UNION SELECT ... FROM ... JOIN cte_name ON ... WHERE ...)
///     DELETE FROM table WHERE table.a = cte_name.a)
///
/// It is mandatory to set the [Self::table_name] and the [Self::query].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonTableExpression {
    pub(crate) table_name: Option<DynIden>,
    pub(crate) cols: Vec<DynIden>,
    pub(crate) query: CteQuery,
    pub(crate) materialized: Option<bool>,
}

impl CommonTableExpression {
    /// Construct a new [`CommonTableExpression`]
    pub fn new() -> CommonTableExpression {
        Self::default()
    }

    /// Sets the CTE table name of the query.
    pub fn table_name<T>(&mut self, table_name: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.table_name = Some(table_name.into_iden());
        self
    }

    /// Sets the CTE VALUES clause.
    ///
    /// It overwrites the query if it is already set for the CTE.
    pub fn values(&mut self, values: Vec<Values>) -> &mut Self {
        self.query = CteQuery::Values(values);
        self
    }

    /// Adds a named column to the CTE table definition.
    pub fn column<C>(&mut self, col: C) -> &mut Self
    where
        C: IntoIden,
    {
        self.cols.push(col.into_iden());
        self
    }

    /// Adds a named columns to the CTE table definition.
    pub fn columns<T, I>(&mut self, cols: I) -> &mut Self
    where
        T: IntoIden,
        I: IntoIterator<Item = T>,
    {
        self.cols
            .extend(cols.into_iter().map(|col| col.into_iden()));
        self
    }

    /// Some databases allow you to put "MATERIALIZED" or "NOT MATERIALIZED" in the CTE definition.
    /// This will affect how during the execution of [WithQuery] the CTE in the [WithClause] will be
    /// executed. If the database doesn't support this syntax this option specified here will be
    /// ignored and not appear in the generated sql.
    pub fn materialized(&mut self, materialized: bool) -> &mut Self {
        self.materialized = Some(materialized);
        self
    }

    /// Set the query generating the CTE content. The query's result must match the defined
    /// columns.
    ///
    /// It overwrites the values if it is already set for the CTE.
    pub fn query<Q>(&mut self, query: Q) -> &mut Self
    where
        Q: Into<SubQueryStatement>,
    {
        self.query = CteQuery::SubQuery(Box::new(query.into()));
        self
    }

    /// Create a CTE from a [SelectStatement] if the selections are named columns then this will
    /// return a [CommonTableExpression] that has the column names set. The [Self::table_name] is
    /// set if the [SelectStatement] from clause contains at least one table.
    pub fn from_select(select: SelectStatement) -> Self {
        let mut cte = Self::default();
        cte.try_set_cols_from_selects(&select.selects);
        if let Some(from) = select.from.first() {
            match from {
                TableRef::Table(_, Some(alias)) => cte.set_table_name_from_select(alias),
                TableRef::Table(TableName(_, tbl), None) => cte.set_table_name_from_select(tbl),
                _ => {}
            }
        }
        cte.query = CteQuery::SubQuery(Box::new(select.into()));
        cte
    }

    fn set_table_name_from_select(&mut self, iden: &DynIden) {
        self.table_name = Some(format!("cte_{iden}").into_iden())
    }

    /// Set up the columns of the CTE to match the given [SelectStatement] selected columns.
    /// This will fail if the select contains non named columns like expressions of wildcards.
    ///
    /// Returns true if the column setup from the select query was successful. If the returned
    /// value is false the columns are untouched.
    pub fn try_set_cols_from_select(&mut self, select: &SelectStatement) -> bool {
        self.try_set_cols_from_selects(&select.selects)
    }

    fn try_set_cols_from_selects(&mut self, selects: &[SelectExpr]) -> bool {
        let vec: Option<Vec<DynIden>> = selects
            .iter()
            .map(|select| {
                if let Some(ident) = &select.alias {
                    Some(ident.clone())
                } else {
                    match &select.expr {
                        Expr::Column(ColumnRef::Column(column_name)) => {
                            // We could depend on `itertools` instead of joining manually.
                            let mut joined_column_name = String::new();
                            for part in column_name.clone().into_iter() {
                                joined_column_name.push_str(&part.0);
                                joined_column_name.push('_');
                            }
                            // Remove the trailing underscore after the column name.
                            joined_column_name.pop();
                            Some(joined_column_name.into_iden())
                        }
                        _ => None,
                    }
                }
            })
            .collect();

        if let Some(c) = vec {
            self.cols = c;
            return true;
        }

        false
    }
}
