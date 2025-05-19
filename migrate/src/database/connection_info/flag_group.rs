use crate::DatabaseConnectionInfo;
use argparse::{ArgumentSource, Error, Flag, FlagGroup, FlagInfo};
use std::path::PathBuf;

const SQLITE_INFO: FlagInfo<PathBuf> = FlagInfo {
    long_name: Some("--sqlite"),
    short_name: None,
    value: Some("PATH"),
    min: Some(1.0),
    max: None,
    default: None,
    description: Some(|_| println!("Sets the path to an SQLite database")),
};

impl FlagGroup for DatabaseConnectionInfo {
    type InProgress = Option<Self>;

    fn new_in_progress() -> Self::InProgress {
        None
    }

    fn parse_long(
        this: &mut Self::InProgress,
        flag: &str,
        source: &mut dyn ArgumentSource,
    ) -> argparse::Result<bool> {
        match flag {
            "sqlite" => {
                if this.is_some() {
                    return Err(Error::repeated_flag(&SQLITE_INFO, true));
                }

                let mut new = None;
                PathBuf::parse(&mut new, source, &SQLITE_INFO, true)?;
                *this = Some(DatabaseConnectionInfo::SQLite(new.unwrap()));
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn unwrap(this: Self::InProgress) -> argparse::Result<Self> {
        match this {
            Some(value) => Ok(value),
            None => Err(Error::missing_argument(SQLITE_INFO.long_name.unwrap())),
        }
    }

    fn print_help_usage() -> bool {
        print!("--sqlite PATH ");
        false
    }

    fn print_help() {
        print!("--sqlite PATH     Sets the path to an SQLite database");
    }

    fn parse_short(
        _: &mut Self::InProgress,
        _: char,
        _: &mut dyn ArgumentSource,
    ) -> argparse::Result<bool> {
        Ok(false)
    }
}
