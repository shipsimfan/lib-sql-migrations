use crate::{DownMigration, UpMigration};
use std::path::{Path, PathBuf};

/// A set of migrations the will be/were applied
pub struct Migrations {
    //// The directory containing the migrations
    base_path: PathBuf,

    /// The up migrations that will be/were applied
    up: Vec<UpMigration>,

    /// The down migrations that will be/were applied
    down: Vec<DownMigration>,

    /// Does/did the "applied_migration" table need to be created?
    table_creation: bool,
}

impl Migrations {
    /// Creates a new set of [`Migrations`], sorting the migrations
    pub(crate) fn new(
        base_path: PathBuf,
        mut up: Vec<UpMigration>,
        mut down: Vec<DownMigration>,
        table_creation: bool,
    ) -> Self {
        up.sort();
        down.sort();

        Migrations {
            base_path,
            up,
            down,
            table_creation,
        }
    }

    /// Gets the directory that contains the migrations
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    /// Gets the up migrations that will be/were applied
    pub fn up(&self) -> &[UpMigration] {
        &self.up
    }

    /// Gets the down migrations that will be/were applied
    pub fn down(&self) -> &[DownMigration] {
        &self.down
    }

    /// Does/did the "applied_migration" table need to be created?
    pub fn table_creation(&self) -> bool {
        self.table_creation
    }

    /// Removes all up migrations
    pub fn clear_up(&mut self) {
        self.up = Vec::new();
    }

    /// Removes all down migrations
    pub fn clear_down(&mut self) {
        self.down = Vec::new();
    }
}
