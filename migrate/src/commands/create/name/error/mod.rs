mod display;
mod new;

/// An invalid migration name was passed
#[derive(Debug)]
pub struct InvalidMigrationName {
    /// The name that was passed
    name: String,
}

impl std::error::Error for InvalidMigrationName {}
