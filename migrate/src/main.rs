use argparse::Command as ArgParseCommand;
use commands::Command;
use database::{Database, DatabaseConnectionInfo};
use options::Options;

mod commands;
mod database;
mod options;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = match Options::parse_env()? {
        Some(options) => options,
        None => return Ok(()),
    };

    options.command.execute()
}
