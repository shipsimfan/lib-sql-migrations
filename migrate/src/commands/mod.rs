use applied::AppliedCommand;
use apply::ApplyCommand;
use apply_down::ApplyDownCommand;
use apply_up::ApplyUpCommand;
use argparse::Positional;
use available::AvailableCommand;
use create::CreateCommand;
use required::RequiredCommand;
use required_down::RequiredDownCommand;
use required_up::RequiredUpCommand;

mod applied;
mod apply;
mod apply_down;
mod apply_up;
mod available;
mod create;
mod required;
mod required_down;
mod required_up;

mod execute;

const DEFAULT_PATH: &str = "migrations";

/// The selected command
#[derive(Debug, Positional)]
pub enum Command {
    /// List the applied migrations
    Applied(AppliedCommand),

    /// Apply all required migrations
    Apply(ApplyCommand),

    /// Apply all required down migrations
    ApplyDown(ApplyDownCommand),

    /// Apply all required up migrations
    ApplyUp(ApplyUpCommand),

    /// Prints the list of applied migrations
    Available(AvailableCommand),

    /// Create a new pair of migration files
    Create(CreateCommand),

    /// List the required migrations
    Required(RequiredCommand),

    /// List the required down migrations
    RequiredDown(RequiredDownCommand),

    /// List the required up migrations
    RequiredUp(RequiredUpCommand),
}
