use crate::{database::OpenDatabase, Database};
use std::path::Path;

/// Lists the migrations that must be applied to `database` from `path`
pub fn required(
    database: Option<&Database>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;

    let migrations = db.get_required_migrations(path)?;

    if migrations.down().len() > 0 {
        println!("Down migrations:");
        for migration in migrations.down() {
            println!("  {}", migration.name());
        }
    }

    if migrations.up().len() > 0 {
        if migrations.down().len() > 0 {
            println!();
        }

        println!("Up migrations:");
        for migration in migrations.up() {
            println!("  {}", migration.name());
        }
    }

    if migrations.down().len() == 0 && migrations.up().len() == 0 {
        println!("No migrations required");
    }

    Ok(())
}
