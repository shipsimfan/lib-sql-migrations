use crate::Database;
use sql_migrations::Migrations;
use std::path::Path;

impl Database {
    /// Gets the migrations that need to be applied to this database
    pub fn get_required_migrations(
        &mut self,
        path: &Path,
    ) -> Result<Migrations, Box<dyn std::error::Error>> {
        match self {
            Database::SQLite(db) => Ok(sql_migrations::get_required_migrations(db, path)?),
        }
    }
}
