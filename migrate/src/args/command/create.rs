use super::Command;
use crate::args::{Options, VERSION};
use argparse::{command, help_flag, parsing_flag, parsing_positional, positionals, version_flag};
use std::{path::PathBuf, str::FromStr};

/// A valid parsed name
struct ValidName(String);

/// An invalid name was passed
#[derive(Debug)]
struct InvalidName(String);

/// Parser for create command
pub const CREATE: argparse::Command<Options> = command! {
    "create" "Creates a new pair of migration files"
    |options: Options| { options.command = Command::Create(String::new()); }
    {
        "Applied Migrations"
        positionals![
            parsing_positional!(
                "NAME" 1*"NAME"
                [
                    "The name of the new migration",
                    "It may only contain lowercase letters, numbers, and dashes"
                ]
                |options: Options, _, name: ValidName| { options.command = Command::Create(name.0); }
            ).set_required(&"missing NAME for create command")
        ]
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

impl FromStr for ValidName {
    type Err = InvalidName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let owned = s.to_string();

        if s.len() == 0 {
            return Err(InvalidName(owned));
        }

        for c in s.chars() {
            if !c.is_ascii_lowercase() && !c.is_digit(10) && c != '-' {
                return Err(InvalidName(owned));
            }
        }

        Ok(ValidName(owned))
    }
}

impl std::error::Error for InvalidName {}

impl std::fmt::Display for InvalidName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid name \"{}\", name must only contain lowercase letters, numbers, and dashes",
            self.0
        )
    }
}
