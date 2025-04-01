use crate::{DownMigration, MigrationError};
use sql::{Connection, Statement, Transaction};

/// Applies `migrations` to `db`
pub(super) fn apply_down_migrations<C: Connection>(
    migrations: &[DownMigration],
    db: &mut C,
) -> Result<(), MigrationError> {
    for migration in migrations {
        apply_down_migration(migration, db).map_err(|error| {
            MigrationError::ApplyDownFailed(error, migration.name().to_string())
        })?;
    }

    Ok(())
}

fn apply_down_migration<C: Connection>(
    migration: &DownMigration,
    db: &mut C,
) -> Result<(), String> {
    let mut transaction = db.begin_trasaction().map_err(|error| error.to_string())?;

    // Get the down migration sql
    let sql = get_sql(migration.id(), &mut transaction)?;

    // Execute the down migration
    transaction
        .execute(&sql)
        .map_err(|error| error.to_string())?;

    // Remove the migration from the "applied_migration" table
    remove_migration(migration.id(), &mut transaction)?;

    transaction.commit().map_err(|error| error.to_string())
}

fn get_sql<'a, T: Transaction<'a>>(id: usize, transaction: &mut T) -> Result<String, String> {
    let mut statement = transaction
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

fn remove_migration<'a, T: Transaction<'a>>(id: usize, transaction: &mut T) -> Result<(), String> {
    let mut statement = transaction
        .prepare("DELETE FROM applied_migration WHERE id = ?")
        .map_err(|error| error.to_string())?;

    statement
        .bind_usize(1, id)
        .map_err(|error| error.to_string())?;

    statement.execute().map_err(|error| error.to_string())
}
