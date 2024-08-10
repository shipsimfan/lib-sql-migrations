use crate::{MigrationError, Migrations};
use sql::Connection;
use std::path::Path;

mod applied;
mod available;

pub use applied::get_applied_migrations;
pub use available::get_available_migrations;

/// Gets the up and down migrations which must be applied to the database
pub fn get_required_migrations<C: Connection>(
    db: &C,
    path: &Path,
) -> Result<Migrations, MigrationError> {
    let mut available_migrations = get_available_migrations(path)?;
    let (mut applied_migrations, table_creation) = get_applied_migrations(db)?
        .map(|applied_migrations| (applied_migrations, false))
        .unwrap_or((Vec::new(), false));

    // Remove any migrations that appear in both the available migrations and the applied
    // migrations as they do not need to be added or removed
    let mut i = 0;
    while i < applied_migrations.len() {
        let applied_migration = &applied_migrations[i];

        let mut found = false;
        for j in 0..available_migrations.len() {
            let available_migration = &available_migrations[i];

            if applied_migration.name() == available_migration.name() {
                available_migrations.swap_remove(j);
                found = true;
                break;
            }
        }

        if found {
            applied_migrations.swap_remove(i);
        } else {
            i += 1;
        }
    }

    Ok(Migrations::new(
        available_migrations,
        applied_migrations,
        table_creation,
    ))
}
