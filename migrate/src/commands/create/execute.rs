use super::CreateMigrationError;
use crate::commands::CreateCommand;
use sql_migrations::{DOWN_SUFFIX, UP_SUFFIX};
use time::{DateTime, NoTimeZone};

impl CreateCommand {
    /// Create a pair of new migrations
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&self.path)
            .map_err(|error| CreateMigrationError::DirectoryFailed(error, self.path.clone()))?;

        let now = DateTime::<NoTimeZone>::now();

        let base_name = format!(
            "{:04}{:02}{:02}-{:02}{:02}-{}",
            now.year(),
            now.month() + 1,
            now.day(),
            now.hour(),
            now.minute(),
            self.name
        );

        let up_name = format!("{}{}", base_name, UP_SUFFIX);
        let down_name = format!("{}{}", base_name, DOWN_SUFFIX);

        let up_path = self.path.join(up_name);
        let down_path = self.path.join(down_name);

        std::fs::File::create(&up_path)
            .map_err(|error| CreateMigrationError::UpFailed(error, up_path.clone()))?;
        println!("Created up migration \"{}\"", up_path.display());

        std::fs::File::create(&down_path).map_err(|error| {
            std::fs::remove_file(&up_path).ok();
            CreateMigrationError::DownFailed(error, down_path.clone())
        })?;
        println!("Created down migration \"{}\"", down_path.display());

        Ok(())
    }
}
