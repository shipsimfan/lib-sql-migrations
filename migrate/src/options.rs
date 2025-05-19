use argparse::Command;

/// The options for the migrations
#[derive(Debug, Command)]
#[command(help, version)]
pub struct Options {
    /// A command to execute. The options are:
    ///  - "applied" displays all currently applied migrations in a database
    ///  - "apply" applies all migrations that need to be applied to a database
    ///  - "apply-down" applies all down migrations that need to be applied to a database
    ///  - "apply-up" applies all up migrations that need to be applied to a database
    ///  - "available" displays all migrations that are available to be applied to a database
    ///  - "create" creates a new pair of migration scripts
    ///  - "required" displays all required migrations to be applied to a database
    ///  - "required-down" displays all required down migrations to be applied to a database
    ///  - "required-up" displays all required up migrations to be applied to a database
    pub command: crate::Command,
}
