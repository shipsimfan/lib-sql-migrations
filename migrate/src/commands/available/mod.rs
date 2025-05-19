use crate::commands::DEFAULT_PATH;
use argparse::Command;
use std::path::PathBuf;

mod execute;

/// Displays the available migrations in `path`
#[derive(Debug, Command)]
#[command(help, description)]
pub struct AvailableCommand {
    /// Sets the path to the migrations
    #[flag(short_name, value = "PATH", default = DEFAULT_PATH.into())]
    path: PathBuf,
}
