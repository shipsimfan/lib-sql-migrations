use argparse::{commands, help_flag, parser, parsing_flag, version_flag};
use command::{
    APPLIED, APPLY, APPLY_DOWN, APPLY_UP, AVAILABLE, CREATE, REQUIRED, REQUIRED_DOWN, REQUIRED_UP,
};
use std::path::{Path, PathBuf};

mod command;
mod database;

pub use command::Command;
pub use database::Database;

/// The options for the migrations
#[derive(Debug, Clone)]
pub struct Options {
    /// The path of the migration files
    migration_path: PathBuf,

    /// The path to an SQLite database
    database: Option<Database>,

    /// The selected command
    command: Command,
}

const VERSION: &str = concat!("sql-migrations v", env!("CARGO_PKG_VERSION"));
const DEFAULT_PATH: &str = "migrations";

parser! { PARSER -> Options
    "sql-migrations"
    "A manager for SQL migrations on databases",
    commands! [
        AVAILABLE,
        APPLIED,
        REQUIRED,
        REQUIRED_DOWN,
        REQUIRED_UP,
        APPLY,
        APPLY_DOWN,
        APPLY_UP,
        CREATE,
    ].required(&"Missing command")
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

impl Options {
    /// Parses the user provided arguments
    pub fn parse() -> Result<Self, argparse::Error<'static>> {
        match PARSER.parse_env(Options::new()) {
            Ok(None) => std::process::exit(0),
            Ok(Some(options)) => Ok(options),
            Err(error) => Err(error),
        }
    }

    /// Creates a new [`Options`] with default settings
    fn new() -> Self {
        Options {
            migration_path: Path::new(DEFAULT_PATH).to_path_buf(),
            database: None,
            command: Command::Available,
        }
    }

    /// Gets the command to run
    pub fn command(&self) -> &Command {
        &self.command
    }

    /// Gets the path to the migrations
    pub fn migration_path(&self) -> &Path {
        &self.migration_path
    }

    /// Gets the options for the database connection
    pub fn database(&self) -> Option<&Database> {
        self.database.as_ref()
    }
}
