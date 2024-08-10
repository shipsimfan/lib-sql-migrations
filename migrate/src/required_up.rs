use crate::{args::Database, database::OpenDatabase};
use std::path::Path;

/// Lists the up migrations that must be applied to `database` from `path`
pub fn required_up(
    database: Option<&Database>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;

    let migrations = db.get_required_migrations(path)?;
    for migration in migrations.up() {
        println!("{}", migration.name());
    }

    Ok(())
}
