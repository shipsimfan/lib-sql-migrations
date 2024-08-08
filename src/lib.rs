//! Implementation for tracking SQL migrations

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use required_migrations::RequiredMigrations;
use sql::Connection;
use std::path::Path;

mod applied_migrations;
mod error;
mod required_migrations;

pub use applied_migrations::AppliedMigrations;
pub use error::ApplyMigrationError;

/// Applies the migrations in the folder at `migrations` to the database connected on `connection`
/// that need to be applied
///
/// Dependencies aren't explicitly stated in the migrations, instead their order is based on the
/// dates baked into their filenames. The implied dependency is that later "down" scripts must be
/// applied before earlier ones and later "up" scripts must be applied after earlier ones.
///
/// To create pairs of migration scripts from a template, run the program that comes with this
/// library, providing it one argument which is the name of the new migration.
pub fn apply_required_migrations<C: Connection, P: AsRef<Path>>(
    connection: &C,
    migrations: P,
) -> Result<AppliedMigrations, ApplyMigrationError> {
    let required_migrations = RequiredMigrations::get(connection, migrations.as_ref())?;

    todo!()
}
