use std::path::PathBuf;

/// The selected database
#[derive(Debug, Clone)]
pub enum Database {
    /// Use an SQLite database
    SQLite(PathBuf),
}
