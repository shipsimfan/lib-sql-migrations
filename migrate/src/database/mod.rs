use sqlite::SQLite3Connection;

mod connection_info;

mod apply_migrations;
mod get_applied_migrations;
mod get_required_migrations;
mod migrate;
mod open;

pub use connection_info::DatabaseConnectionInfo;

/// An open database connection
pub enum Database {
    /// Connected to an SQLite database
    SQLite(SQLite3Connection),
}
