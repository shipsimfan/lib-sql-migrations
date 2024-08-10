use std::path::PathBuf;

/// An error that can occur while migrating a database
#[derive(Debug)]
pub enum MigrationError {
    /// Opening the migrations folder failed
    OpenMigrationsFailed(std::io::Error, PathBuf),

    /// Failed to check if the "applied_migration" table exists
    TableCheckFailed(String),

    /// Failed to get the applied migrations
    GetAppliedMigrationsFailed(String),
}

impl std::error::Error for MigrationError {}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::OpenMigrationsFailed(error, path) => write!(
                f,
                "unable to open migrations directory \"{}\" - {}",
                path.display(),
                error
            ),
            MigrationError::TableCheckFailed(error) => {
                write!(f, "unable to check if the table exists - {}", error)
            }
            MigrationError::GetAppliedMigrationsFailed(error) => {
                write!(f, "unable to get the applied migrations - {}", error)
            }
        }
    }
}
