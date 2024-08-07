//! Implementation for tracking SQL migrations

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

use sql::Connection;
use std::path::Path;

mod applied_migrations;
mod error;

pub use applied_migrations::AppliedMigrations;
pub use error::ApplyMigrationError;

/// Applies the migrations in the folder at `migrations` to the database connected on `connection`
pub fn apply_migrations<C: Connection, P: AsRef<Path>>(
    connection: &mut C,
    migrations: P,
) -> Result<AppliedMigrations, ApplyMigrationError> {
    todo!()
}
