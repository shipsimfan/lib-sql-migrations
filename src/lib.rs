//! Implementation for tracking SQL migrations

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use sql::Connection;
use std::path::Path;

mod apply;
mod error;
mod get;
mod migration;
mod migrations;

pub use apply::apply_migrations;
pub use error::MigrationError;
pub use get::{get_applied_migrations, get_available_migrations, get_required_migrations};
pub use migration::{DownMigration, UpMigration};
pub use migrations::Migrations;

/// Applies the migrations in the folder at `migrations` to the database connected on `db` that
/// need to be applied
///
/// Dependencies aren't explicitly stated in the migrations, instead their order is based on the
/// dates baked into their filenames. The implied dependency is that later "down" scripts must be
/// applied before earlier ones and later "up" scripts must be applied after earlier ones.
///
/// To create pairs of migration scripts from a template, run the program that comes with this
/// library.
pub fn migrate<C: Connection, P: AsRef<Path>>(
    db: &C,
    path: P,
) -> Result<Migrations, MigrationError> {
    let migrations = get_required_migrations(db, path.as_ref())?;

    apply_migrations(&migrations, db)?;

    Ok(migrations)
}
