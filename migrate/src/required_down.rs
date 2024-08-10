use crate::{args::Database, database::OpenDatabase};
use std::path::Path;

/// Lists the down migrations that must be applied to `database` from `path`
pub fn required_down(
    database: Option<&Database>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;

    let migrations = db.get_required_migrations(path)?;
    for migration in migrations.down() {
        println!("{}", migration.name());
    }

    Ok(())
}
