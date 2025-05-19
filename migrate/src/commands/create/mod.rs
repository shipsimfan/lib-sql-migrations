use crate::commands::DEFAULT_PATH;
use argparse::Command;
use error::CreateMigrationError;
use name::MigrationName;
use std::path::PathBuf;

mod error;
mod name;

mod execute;

/// Creates a pair of new migrations
#[derive(Debug, Command)]
#[command(help, description)]
pub struct CreateCommand {
    /// The name for the new migration
    name: MigrationName,

    /// Sets the path to the migrations
    #[flag(short_name, value = "PATH", default = DEFAULT_PATH.into())]
    path: PathBuf,
}
