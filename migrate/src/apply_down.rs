use crate::{database::OpenDatabase, Database};
use std::path::Path;

/// Applies the required down migrations to `database`
pub fn apply_down(
    database: Option<&Database>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;

    let mut migrations = db.get_required_migrations(path)?;
    migrations.clear_up();
    db.apply_migrations(&migrations)
}
