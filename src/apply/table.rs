use crate::MigrationError;
use sql::Connection;

const SQL: &str = include_str!("create-applied-migrations.sql");

/// Creates the `applied_migration` table in `db`
pub(super) fn create_table<C: Connection>(db: &C) -> Result<(), MigrationError> {
    db.execute(SQL)
        .map_err(|error| MigrationError::CreateTableFailed(error.to_string()))
}
