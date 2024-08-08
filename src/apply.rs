use crate::ApplyMigrationError;
use sql::Connection;
use std::path::{Path, PathBuf};

/// Applies `migrations` to the database
pub(super) fn apply_migrations<C: Connection>(
    connection: &C,
    migrations: &[PathBuf],
    up: bool,
) -> Result<(), ApplyMigrationError> {
    for migration in migrations {
        apply_migration(connection, migration, up)?;
    }

    Ok(())
}

fn apply_migration<C: Connection>(
    connection: &C,
    migration: &Path,
    up: bool,
) -> Result<(), ApplyMigrationError> {
    let mut sql = std::fs::read_to_string(migration).map_err(|error| {
        ApplyMigrationError::ReadMigrationFileFailed(error, migration.to_path_buf())
    })?;

    if up {
        sql = augment_up_sql(sql, migration);
    } else {
        augment_down_sql(&mut sql, migration);
    }

    connection.execute(&sql).map_err(|error| {
        ApplyMigrationError::ApplyMigrationFailed(error.to_string(), migration.to_path_buf())
    })
}

fn augment_down_sql(sql: &mut String, path: &Path) {
    let migration_name = &path.file_stem().unwrap().to_string_lossy();

    sql.push_str("\nDELETE FROM applied_migration WHERE name = '");
    sql.push_str(&migration_name);
    sql.push_str("';");
}

fn augment_up_sql(original: String, path: &Path) -> String {
    let migration_name = &path.file_stem().unwrap().to_string_lossy();

    let mut sql = "BEGIN TRANSACTION;\n".to_string();

    sql.push_str(&original);
    sql.push('\n');

    sql.push_str("INSERT INTO applied_migration (name) VALUES ('");
    sql.push_str(&migration_name);
    sql.push_str("');\n");

    sql.push_str("END TRANSACTION;");

    sql
}
