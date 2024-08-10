use crate::{MigrationError, Migrations};
use down::apply_down_migrations;
use sql::Connection;
use table::create_table;
use up::apply_up_migrations;

mod down;
mod table;
mod up;

/// Applies `migrations` to `db`
pub fn apply_migrations<C: Connection>(
    migrations: &Migrations,
    db: &C,
) -> Result<(), MigrationError> {
    if migrations.table_creation() {
        create_table(db)?;
    }

    apply_down_migrations(migrations.down(), db)?;
    apply_up_migrations(migrations.base_path(), migrations.up(), db)?;

    Ok(())
}
