use crate::DatabaseConnectionInfo;
use argparse::Command;

mod execute;

/// Prints the list of applied migrations
#[derive(Debug, Command)]
#[command(help, description)]
pub struct AppliedCommand {
    /// The selected database to use
    #[flag_group]
    database: DatabaseConnectionInfo,
}
