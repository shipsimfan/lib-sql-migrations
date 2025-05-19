use super::CreateMigrationError;

impl std::fmt::Display for CreateMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateMigrationError::DirectoryFailed(error, path) => write!(
                f,
                "unable to create the migration directory \"{}\" - {}",
                path.display(),
                error
            ),
            CreateMigrationError::UpFailed(error, path) => write!(
                f,
                "unable to create the up migration script \"{}\" - {}",
                path.display(),
                error
            ),
            CreateMigrationError::DownFailed(error, path) => write!(
                f,
                "unable to create the down migration script \"{}\" - {}",
                path.display(),
                error
            ),
        }
    }
}
