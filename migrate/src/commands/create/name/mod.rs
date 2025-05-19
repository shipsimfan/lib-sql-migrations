use error::InvalidMigrationName;

mod error;

mod display;
mod positional;

/// A valid migration name
#[derive(Debug)]
pub struct MigrationName {
    /// The name itself
    name: String,
}
