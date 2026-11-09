use super::*;

impl UpdateStatement {
    /// Get column values
    pub fn get_values(&self) -> &[(DynIden, Box<Expr>)] {
        &self.values
    }

    /// The table targeted by this update.
    pub fn get_table(&self) -> Option<&TableRef> {
        self.table.as_deref()
    }
    /// Additional sources used by this update.
    pub fn get_from(&self) -> &[TableRef] {
        &self.from
    }
    /// Predicates, including their original logical chains.
    pub fn get_where(&self) -> &ConditionHolder {
        &self.r#where
    }
    /// Ordering used to choose rows.
    pub fn get_orders(&self) -> &[OrderExpr] {
        &self.orders
    }
    /// The maximum number of rows to update.
    pub fn get_limit(&self) -> Option<u64> {
        match self.limit {
            Some(Value::BigUnsigned(Some(limit))) => Some(limit),
            _ => None,
        }
    }
    /// Common table expressions attached to this update.
    pub fn get_with(&self) -> Option<&WithClause> {
        self.with.as_ref()
    }
}
