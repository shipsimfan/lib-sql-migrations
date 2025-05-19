use crate::Database;
use sql_migrations::Migrations;

impl Database {
    /// Apply the required migrations to the database
    pub fn apply_migrations(
        &mut self,
        migrations: &Migrations,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Database::SQLite(db) => {
                Ok(sql_migrations::apply_migrations(migrations, db).map(|_| ())?)
            }
        }
    }
}
