use sql_migrations::{DOWN_SUFFIX, UP_SUFFIX};
use std::path::{Path, PathBuf};
use time::{DateTime, NoTimeZone};

#[derive(Debug)]
pub enum CreateMigrationError {
    DirectoryFailed(std::io::Error, PathBuf),
    UpFailed(std::io::Error, PathBuf),
    DownFailed(std::io::Error, PathBuf),
}

/// Create a pair of new migrations
pub fn create(path: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path)
        .map_err(|error| CreateMigrationError::DirectoryFailed(error, path.to_path_buf()))?;

    let now = DateTime::<NoTimeZone>::now();

    let base_name = format!(
        "{:04}{:02}{:02}-{:02}{:02}-{}",
        now.year(),
        now.month() + 1,
        now.day(),
        now.hour(),
        now.minute(),
        name
    );

    let up_name = format!("{}{}", base_name, UP_SUFFIX);
    let down_name = format!("{}{}", base_name, DOWN_SUFFIX);

    let up_path = path.join(up_name);
    let down_path = path.join(down_name);

    std::fs::File::create(&up_path)
        .map_err(|error| CreateMigrationError::UpFailed(error, up_path.clone()))?;
    println!("Created up migration \"{}\"", up_path.display());

    std::fs::File::create(&down_path)
        .map_err(|error| CreateMigrationError::DownFailed(error, down_path.clone()))?;
    println!("Created down migration \"{}\"", down_path.display());

    Ok(())
}

impl std::error::Error for CreateMigrationError {}

impl std::fmt::Display for CreateMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateMigrationError::DirectoryFailed(error, path) => write!(
                f,
                "unable to create the migration directory \"{}\" - {}",
                path.display(),
                error
            ),
            CreateMigrationError::UpFailed(error, path) => write!(
                f,
                "unable to create the up migration script \"{}\" - {}",
                path.display(),
                error
            ),
            CreateMigrationError::DownFailed(error, path) => write!(
                f,
                "unable to create the down migration script \"{}\" - {}",
                path.display(),
                error
            ),
        }
    }
}
