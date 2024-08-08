use std::path::PathBuf;

/// An error that can occur while apply migrations to a database
#[derive(Debug)]
pub enum ApplyMigrationError {
    /// Getting the available migrations failed
    GetAvailableMigrationsFailed(std::io::Error, PathBuf),

    /// Failed to get the applied migrations
    GetAppliedMigrationsFailed(String),
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
        }
    }
}
