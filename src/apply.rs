use crate::db::DatabaseOpenError;
use sqlite::{sql::Connection, SQLite3Connection};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum ApplyMigrationError {
    File(std::io::Error),
    SQL(String),
}

/// Applies `migrations` to the database
pub(super) fn apply_migrations(
    connection: &SQLite3Connection,
    migrations: Vec<PathBuf>,
) -> Result<(), DatabaseOpenError> {
    for migration in migrations {
        apply_migration(connection, &migration)
            .map_err(|error| DatabaseOpenError::ApplyMigrationFailed(error, migration))?;
    }

    Ok(())
}

fn apply_migration(
    connection: &SQLite3Connection,
    migration: &Path,
) -> Result<(), ApplyMigrationError> {
    let sql = std::fs::read_to_string(migration).map_err(ApplyMigrationError::File)?;

    connection.execute(&sql).map_err(ApplyMigrationError::SQL)
}

impl Display for ApplyMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyMigrationError::File(error) => error.fmt(f),
            ApplyMigrationError::SQL(error) => error.fmt(f),
        }
    }
}
