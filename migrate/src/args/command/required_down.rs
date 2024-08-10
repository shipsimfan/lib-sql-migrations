use crate::{args::VERSION, Command, Database, Options};
use argparse::{command, help_flag, parsing_flag, version_flag};
use std::path::PathBuf;

/// Parser for required down command
pub const REQUIRED_DOWN: argparse::Command<Options> = command! {
    "required-down" "Prints the list of required down migrations"
    |options: Options| { options.command = Command::RequiredDown; }
    {
        "Required Down Migrations"
        [
            parsing_flag!(
                "p", "path" "PATH" "missing PATH for --path"
                [
                    "Sets the path to search for migrations",
                    "Defaults to \"migrations\""
                ]
                |options: Options, path: PathBuf| { options.migration_path = path; }
            ),
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
