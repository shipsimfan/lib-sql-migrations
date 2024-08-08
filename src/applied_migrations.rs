use crate::RequiredMigrations;
use std::path::PathBuf;

/// A set of applied migrations
#[derive(Debug, PartialEq, Eq)]
pub struct AppliedMigrations {
    /// The up migrations applying changes to the database
    up: Vec<String>,

    /// The down migrations, undoing previously applied migrations
    down: Vec<String>,
}

impl AppliedMigrations {
    /// Gets the applied up migrations
    pub fn up(&self) -> &[String] {
        &self.up
    }

    /// Gets the applied down migrations
    pub fn down(&self) -> &[String] {
        &self.down
    }
}

impl From<RequiredMigrations> for AppliedMigrations {
    fn from(required_migrations: RequiredMigrations) -> Self {
        fn convert_list(list: &[PathBuf]) -> Vec<String> {
            list.iter()
                .map(|path| path.file_stem().unwrap().to_string_lossy().to_string())
                .collect()
        }

        let up = convert_list(required_migrations.up());
        let down = convert_list(required_migrations.down());

        AppliedMigrations { up, down }
    }
}
