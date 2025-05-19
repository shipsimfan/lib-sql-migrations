use crate::Database;
use sql_migrations::DownMigration;

impl Database {
    /// Gets the migrations that have been applied to this database
    pub fn get_applied_migrations(
        &mut self,
    ) -> Result<Option<Vec<DownMigration>>, Box<dyn std::error::Error>> {
        match self {
            Database::SQLite(db) => Ok(sql_migrations::get_applied_migrations(db)?),
        }
    }
}
