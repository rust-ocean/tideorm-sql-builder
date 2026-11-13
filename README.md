# tideorm-sql-builder

The SQL statement builder TideORM's engine renders with (lib `sql_builder`),
re-exported as `tideorm::orm::sql_builder`: statements, conditions, values and
the per-backend query builders and dialects for PostgreSQL, MySQL, MariaDB
(`MariaDbDialect`, MySQL's builder where the servers agree) and SQLite.

The workspace also holds `tideorm-sql-builder-derive` (lib `sql_builder_derive`),
the proc-macro crate behind the builder's `Iden` and `IdenStatic` derives. It
builds on its own: no sibling checkout is needed.

## Tests

```bash
cargo +1.99.0 test --workspace --all-features
```
