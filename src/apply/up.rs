use crate::{MigrationError, UpMigration};
use sql::{Connection, Statement};
use std::path::Path;

/// Applies `migrations` to `db`
pub(super) fn apply_up_migrations<C: Connection>(
    base_path: &Path,
    migrations: &[UpMigration],
    db: &C,
) -> Result<(), MigrationError> {
    for migration in migrations {
        apply_up_migration(base_path, migration, db)?;
    }

    Ok(())
}

fn apply_up_migration<C: Connection>(
    base_path: &Path,
    migration: &UpMigration,
    db: &C,
) -> Result<(), MigrationError> {
    // Load the "up" sql
    let mut sql = std::fs::read_to_string(migration.up_source()).map_err(|error| {
        MigrationError::ReadUpFailed(error, migration.up_source().to_path_buf())
    })?;
    sql = wrap_transaction(sql);

    // Load the "down" sql
    let down_path = migration.down_source(base_path);
    let down_sql = std::fs::read_to_string(&down_path)
        .map_err(|error| MigrationError::ReadDownFailed(error, down_path))?;

    // Execute the sql
    db.execute(&sql).map_err(|error| {
        MigrationError::ApplyUpFailed(error.to_string(), migration.name().to_string())
    })?;

    // Insert the new migration into the migrations table
    insert_new_migration(migration.name(), &down_sql, db).map_err(|error| {
        MigrationError::ApplyUpFailed(error.to_string(), migration.name().to_string())
    })
}

fn wrap_transaction(sql: String) -> String {
    format!("BEGIN TRANSACTION;\n{}\nCOMMIT;", sql)
}

fn insert_new_migration<C: Connection>(name: &str, down_sql: &str, db: &C) -> Result<(), String> {
    let mut statement = db
        .prepare("INSERT INTO applied_migration (name, down_sql) VALUES (?, ?)")
        .map_err(|error| error.to_string())?;

    statement
        .bind_str(1, name)
        .map_err(|error| error.to_string())?;
    statement
        .bind_str(2, down_sql)
        .map_err(|error| error.to_string())?;

    statement.execute().map_err(|error| error.to_string())
}
