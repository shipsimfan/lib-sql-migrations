use crate::Database;
use std::path::Path;

impl Database {
    /// Apply the required migrations to the database
    pub fn migrate(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Database::SQLite(db) => Ok(sql_migrations::migrate(db, path).map(|_| ())?),
        }
    }
}
