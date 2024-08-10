use args::{Command, Database, Options};

mod applied;
mod args;
mod available;
mod database;
mod required;
mod required_down;
mod required_up;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse()?;

    match options.command() {
        Command::Available => available::available(options.migration_path()),
        Command::Applied => applied::applied(options.database()),
        Command::Required => required::required(options.database(), options.migration_path()),
        Command::RequiredUp => {
            required_up::required_up(options.database(), options.migration_path())
        }
        Command::RequiredDown => {
            required_down::required_down(options.database(), options.migration_path())
        }
        Command::Apply => todo!(),
    }
}
