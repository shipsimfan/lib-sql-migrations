use crate::{database::OpenDatabase, Database};
use std::path::Path;

/// Applies the required migrations to `database`
pub fn apply(database: Option<&Database>, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;
    db.migrate(path)
}
