use crate::{DownMigration, MigrationError};
use sql::{Connection, Statement};

/// Gets the migrations which have already been applied to `db`, returning [`None`] if the
/// "applied_migration" table needs to be created
pub fn get_applied_migrations<C: Connection>(
    db: &C,
) -> Result<Option<Vec<DownMigration>>, MigrationError> {
    if !does_table_exist(db).map_err(|error| MigrationError::TableCheckFailed(error))? {
        return Ok(None);
    }

    db.prepare("SELECT id, name FROM applied_migration")
        .map_err(|error| MigrationError::GetAppliedMigrationsFailed(error.to_string()))?
        .rows::<DownMigration>()
        .map_err(|error| MigrationError::GetAppliedMigrationsFailed(error.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map(|migrations| Some(migrations))
        .map_err(|error| MigrationError::GetAppliedMigrationsFailed(error.to_string()))
}

/// Checks if the "applied_migration" table has been created
fn does_table_exist<C: Connection>(db: &C) -> Result<bool, String> {
    let mut rows = db
        .prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='applied_migration';",
        )
        .map_err(|error| error.to_string())?
        .rows()
        .map_err(|error| error.to_string())?;

    let count = rows.next().map(|row| row.unwrap_or(0)).unwrap_or(0);
    Ok(count > 0)
}
