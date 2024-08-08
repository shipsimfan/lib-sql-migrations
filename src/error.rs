use std::path::PathBuf;

/// An error that can occur while apply migrations to a database
#[derive(Debug)]
pub enum ApplyMigrationError {
    /// Getting the available migrations failed
    GetAvailableMigrationsFailed(std::io::Error, PathBuf),

    /// Failed to get the applied migrations
    GetAppliedMigrationsFailed(String),

    /// Failed to create the applied migrations table
    CreateAppliedMigrationsTableFailed(String),

    /// Failed to read a migration file
    ReadMigrationFileFailed(std::io::Error, PathBuf),

    /// Failed to apply a migration
    ApplyMigrationFailed(String, PathBuf),
}

impl std::error::Error for ApplyMigrationError {}

impl std::fmt::Display for ApplyMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyMigrationError::GetAvailableMigrationsFailed(error, path) => write!(
                f,
                "unable to get the available migrations at \"{}\" - {}",
                path.display(),
                error
            ),
            ApplyMigrationError::GetAppliedMigrationsFailed(error) => write!(
                f,
                "unable to get the currently applied migrations - {}",
                error
            ),
            ApplyMigrationError::ReadMigrationFileFailed(error, path) => write!(
                f,
                "unable to read migration \"{}\" - {}",
                path.display(),
                error
            ),
            ApplyMigrationError::ApplyMigrationFailed(error, path) => write!(
                f,
                "unable to apply migration \"{}\" - {}",
                path.display(),
                error
            ),
            ApplyMigrationError::CreateAppliedMigrationsTableFailed(error) => {
                write!(
                    f,
                    "unable to create the \"applied_migrations\" table - {}",
                    error
                )
            }
        }
    }
}
