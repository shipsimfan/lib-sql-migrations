/// An error that can occur while apply migrations to a database
#[derive(Debug)]
pub enum ApplyMigrationError {}

impl std::error::Error for ApplyMigrationError {}

impl std::fmt::Display for ApplyMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
