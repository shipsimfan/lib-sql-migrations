use std::path::PathBuf;

mod display;

/// An error that can occur while creating a migration
#[derive(Debug)]
pub enum CreateMigrationError {
    /// Couldn't open the migration directory
    DirectoryFailed(std::io::Error, PathBuf),

    /// Couldn't create the up-migration
    UpFailed(std::io::Error, PathBuf),

    /// Couldn't create the down-migration
    DownFailed(std::io::Error, PathBuf),
}

impl std::error::Error for CreateMigrationError {}
