use std::path::Path;

use crate::args::Database;
use sql_migrations::{DownMigration, Migrations};
use sqlite::SQLite3Connection;

/// An open database connection
pub enum OpenDatabase {
    /// Connected to an SQLite database
    SQLite(SQLite3Connection),
}

/// No database was specified to be opened
#[derive(Debug, Clone, Copy)]
pub struct NoDatabaseSpecified;

impl OpenDatabase {
    /// Open the database
    pub fn open(database: Option<&Database>) -> Result<Self, Box<dyn std::error::Error>> {
        match database {
            Some(Database::SQLite(path)) => {
                Ok(OpenDatabase::SQLite(SQLite3Connection::open(path)?))
            }
            None => return Err(Box::new(NoDatabaseSpecified)),
        }
    }

    /// Gets the migrations that have been applied to this database
    pub fn get_applied_migrations(
        &self,
    ) -> Result<Option<Vec<DownMigration>>, Box<dyn std::error::Error>> {
        match self {
            OpenDatabase::SQLite(db) => Ok(sql_migrations::get_applied_migrations(db)?),
        }
    }

    /// Gets the migrations that need to be applied to this database
    pub fn get_required_migrations(
        &self,
        path: &Path,
    ) -> Result<Migrations, Box<dyn std::error::Error>> {
        match self {
            OpenDatabase::SQLite(db) => Ok(sql_migrations::get_required_migrations(db, path)?),
        }
    }

    /// Apply the required migrations to the database
    pub fn migrate(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            OpenDatabase::SQLite(db) => Ok(sql_migrations::migrate(db, path).map(|_| ())?),
        }
    }

    /// Apply the required migrations to the database
    pub fn apply_migrations(
        &self,
        migrations: &Migrations,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            OpenDatabase::SQLite(db) => {
                Ok(sql_migrations::apply_migrations(migrations, db).map(|_| ())?)
            }
        }
    }
}

impl std::error::Error for NoDatabaseSpecified {}

impl std::fmt::Display for NoDatabaseSpecified {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("no database specified")
    }
}
