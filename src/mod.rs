use super::DatabaseOpenError;
use apply::apply_migrations;
use get_required::get_required_migrations;
use oak::Logger;
use sqlite::SQLite3Connection;

mod apply;
mod get_required;

pub use apply::ApplyMigrationError;

pub(super) fn apply_required_migrations(
    connection: &SQLite3Connection,
    logger: &Logger,
) -> Result<(), DatabaseOpenError> {
    let required_migrations = get_required_migrations(connection, logger)?;

    apply_migrations(connection, required_migrations.down)?;
    apply_migrations(connection, required_migrations.up)?;

    Ok(())
}
