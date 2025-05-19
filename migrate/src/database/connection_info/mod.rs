use std::path::PathBuf;

mod flag_group;

/// The selected database
#[derive(Debug, Clone)]
pub enum DatabaseConnectionInfo {
    /// Use an SQLite database
    SQLite(PathBuf),
}
