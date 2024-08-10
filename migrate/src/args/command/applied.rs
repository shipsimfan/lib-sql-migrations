use crate::{args::VERSION, Command, Database, Options};
use argparse::{command, help_flag, parsing_flag, version_flag};
use std::path::PathBuf;

/// Parser for applied command
pub const APPLIED: argparse::Command<Options> = command! {
    "applied" "Prints the list of applied migrations"
    |options: Options| { options.command = Command::Applied; }
    {
        "Applied Migrations"
        [
            parsing_flag!(
                , "sqlite" "PATH" "missing PATH for --sqlite"
                "Uses the SQLite database at PATH as the target database"
                |options: Options, path: PathBuf| { options.database = Some(Database::SQLite(path)); }
            ),
            help_flag!("h", "help"),
            version_flag!("v", "version" VERSION)
        ]
    }
};
