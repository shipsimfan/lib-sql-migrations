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

    /// Failed to create the "applied_migration" table
    CreateTableFailed(String),

    /// Failed to apply a down migration
    ApplyDownFailed(String, String),

    /// Failed to read an up migration
    ReadUpFailed(std::io::Error, PathBuf),

    /// Failed to read a down migration
    ReadDownFailed(std::io::Error, PathBuf),

    /// Failed to apply an up migration
    ApplyUpFailed(String, String),
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
                write!(
                    f,
                    "unable to check if the \"applied_migration\" table exists - {}",
                    error
                )
            }
            MigrationError::GetAppliedMigrationsFailed(error) => {
                write!(f, "unable to get the applied migrations - {}", error)
            }
            MigrationError::CreateTableFailed(error) => write!(
                f,
                "unable to create the \"applied_migration\" table - {}",
                error
            ),
            MigrationError::ApplyDownFailed(error, name) => {
                write!(f, "unable to apply down migration \"{}\" - {}", name, error)
            }
            MigrationError::ReadUpFailed(error, path) => {
                write!(
                    f,
                    "unable to read up migration \"{}\" - {}",
                    path.display(),
                    error
                )
            }
            MigrationError::ReadDownFailed(error, path) => {
                write!(
                    f,
                    "unable to read down migration \"{}\" - {}",
                    path.display(),
                    error
                )
            }
            MigrationError::ApplyUpFailed(error, name) => {
                write!(f, "unable to apply up migration \"{}\" - {}", name, error)
            }
        }
    }
}
