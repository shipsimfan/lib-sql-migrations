use crate::{args::VERSION, Command, Options};
use argparse::{command, help_flag, parsing_flag, version_flag};
use std::path::PathBuf;

/// Parser for available command
pub const AVAILABLE: argparse::Command<Options> = command! {
    "available" "Prints the list of available migrations"
    |options: Options| { options.command = Command::Available; }
    {
        "Available Migrations"
        [
            parsing_flag!(
                "p", "path" "PATH" "missing PATH for --path"
                [
                    "Sets the path to search for migrations",
                    "Defaults to \"migrations\""
                ]
                |options: Options, path: PathBuf| { options.migration_path = path; }
            ),
            help_flag!("h", "help"),
            version_flag!("v", "version" VERSION)
        ]
    }
};
