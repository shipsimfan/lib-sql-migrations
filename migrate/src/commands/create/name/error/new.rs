use super::InvalidMigrationName;

impl InvalidMigrationName {
    /// Creates a new [`InvalidMigrationName`]
    pub fn new(name: String) -> Self {
        InvalidMigrationName { name }
    }
}
