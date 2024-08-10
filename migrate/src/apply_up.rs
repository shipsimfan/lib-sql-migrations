use crate::{database::OpenDatabase, Database};
use std::path::Path;

/// Applies the required up migrations to `database`
pub fn apply_up(
    database: Option<&Database>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;

    let mut migrations = db.get_required_migrations(path)?;
    migrations.clear_down();
    db.apply_migrations(&migrations)
}
