use crate::{Database, DatabaseConnectionInfo};
use sqlite::SQLite3Connection;

impl Database {
    /// Open the database
    pub fn open(database: DatabaseConnectionInfo) -> Result<Self, Box<dyn std::error::Error>> {
        match database {
            DatabaseConnectionInfo::SQLite(path) => {
                Ok(Database::SQLite(SQLite3Connection::open(path)?))
            }
        }
    }
}
