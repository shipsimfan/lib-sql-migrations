use crate::{DownMigration, MigrationError};
use sql::{Connection, Statement};

/// Applies `migrations` to `db`
pub(super) fn apply_down_migrations<C: Connection>(
    migrations: &[DownMigration],
    db: &C,
) -> Result<(), MigrationError> {
    for migration in migrations {
        apply_down_migration(migration, db).map_err(|error| {
            MigrationError::ApplyDownFailed(error, migration.name().to_string())
        })?;
    }

    Ok(())
}

fn apply_down_migration<C: Connection>(migration: &DownMigration, db: &C) -> Result<(), String> {
    // Get the down migration sql
    let sql = get_sql(migration.id(), db)?;

    // Execute the down migration
    db.execute(&sql).map_err(|error| error.to_string())?;

    // Remove the migration from the "applied_migration" table
    remove_migration(migration.id(), db)
}

fn get_sql<C: Connection>(id: usize, db: &C) -> Result<String, String> {
    let mut statement = db
        .prepare("SELECT down_sql FROM applied_migration WHERE id = ?")
        .map_err(|error| error.to_string())?;

    statement
        .bind_usize(1, id)
        .map_err(|error| error.to_string())?;

    statement
        .rows()
        .map_err(|error| error.to_string())?
        .next()
        .unwrap_or(Ok(String::new()))
        .map_err(|error| error.to_string())
}

fn remove_migration<C: Connection>(id: usize, db: &C) -> Result<(), String> {
    let mut statement = db
        .prepare("DELETE FROM applied_migration WHERE id = ?")
        .map_err(|error| error.to_string())?;

    statement
        .bind_usize(1, id)
        .map_err(|error| error.to_string())?;

    statement.execute().map_err(|error| error.to_string())
}
