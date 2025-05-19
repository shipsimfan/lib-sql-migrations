use crate::{commands::DEFAULT_PATH, DatabaseConnectionInfo};
use argparse::Command;
use std::path::PathBuf;

mod execute;

/// Applies the required down migrations to `database`
#[derive(Debug, Command)]
#[command(help, description)]
pub struct ApplyDownCommand {
    /// The selected database to use
    #[flag_group]
    database: DatabaseConnectionInfo,

    /// Sets the path to the migrations
    #[flag(short_name, value = "PATH", default = DEFAULT_PATH.into())]
    path: PathBuf,
}
