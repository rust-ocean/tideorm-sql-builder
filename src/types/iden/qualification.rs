//! Names that may carry qualifiers: what a bare identifier or a tuple of them becomes
//! when a schema, type, table or column name is asked for.

use super::*;

/// A name with at most one qualifier: `name` or `qualifier.name`.
///
/// Its implementations exist to give the name types their `From` conversions.
pub trait MaybeQualifiedOnce {
    /// Splits the name into `(qualifier, name)`.
    fn into_2_parts(self) -> (Option<DynIden>, DynIden);
}

/// A bare identifier: `name`.
impl<T> MaybeQualifiedOnce for T
where
    T: IntoIden,
{
    fn into_2_parts(self) -> (Option<DynIden>, DynIden) {
        (None, self.into_iden())
    }
}

/// A pair: `qualifier.name`.
impl<S, T> MaybeQualifiedOnce for (S, T)
where
    S: IntoIden,
    T: IntoIden,
{
    fn into_2_parts(self) -> (Option<DynIden>, DynIden) {
        let (qualifier, name) = self;
        (Some(qualifier.into_iden()), name.into_iden())
    }
}

/// A name with up to two qualifiers: `name`, `inner.name` or `outer.inner.name`.
///
/// Its implementations exist to give the name types their `From` conversions.
pub trait MaybeQualifiedTwice {
    /// Splits the name into `((outer, inner), name)`; nesting the qualifiers this way
    /// leaves no room for an outer one without an inner one.
    fn into_3_parts(self) -> (Option<(Option<DynIden>, DynIden)>, DynIden);
}

/// A bare identifier or a pair, whose qualifier is the inner one.
impl<T> MaybeQualifiedTwice for T
where
    T: MaybeQualifiedOnce,
{
    fn into_3_parts(self) -> (Option<(Option<DynIden>, DynIden)>, DynIden) {
        let (inner, name) = self.into_2_parts();
        (inner.map(|inner| (None, inner)), name)
    }
}

/// A triple: `outer.inner.name`.
impl<S, T, U> MaybeQualifiedTwice for (S, T, U)
where
    S: IntoIden,
    T: IntoIden,
    U: IntoIden,
{
    fn into_3_parts(self) -> (Option<(Option<DynIden>, DynIden)>, DynIden) {
        let (outer, inner, name) = self;
        let qualifiers = (Some(outer.into_iden()), inner.into_iden());
        (Some(qualifiers), name.into_iden())
    }
}

/// `(database, schema)` as a [`SchemaName`].
fn schema_name((database, schema): (Option<DynIden>, DynIden)) -> SchemaName {
    SchemaName(database.map(DatabaseName), schema)
}

/// `schema` or `database.schema`.
impl<T> From<T> for SchemaName
where
    T: MaybeQualifiedOnce,
{
    fn from(value: T) -> Self {
        schema_name(value.into_2_parts())
    }
}

/// `type`, `schema.type` or `database.schema.type`.
impl<T> From<T> for TypeRef
where
    T: MaybeQualifiedTwice,
{
    fn from(value: T) -> Self {
        let (schema, name) = value.into_3_parts();
        TypeRef(schema.map(schema_name), name)
    }
}

/// `table`, `schema.table` or `database.schema.table`.
impl<T> From<T> for TableName
where
    T: MaybeQualifiedTwice,
{
    fn from(value: T) -> Self {
        let (schema, name) = value.into_3_parts();
        TableName(schema.map(schema_name), name)
    }
}

/// `column`, `table.column` or `schema.table.column`.
impl<T> From<T> for ColumnName
where
    T: MaybeQualifiedTwice,
{
    fn from(value: T) -> Self {
        let (table, name) = value.into_3_parts();
        let table = table
            .map(|(schema, table)| TableName(schema.map(|schema| SchemaName(None, schema)), table));
        ColumnName(table, name)
    }
}
