use crate::{DownMigration, UpMigration};

/// A set of migrations the will be/were applied
pub struct Migrations {
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
        mut up: Vec<UpMigration>,
        mut down: Vec<DownMigration>,
        table_creation: bool,
    ) -> Self {
        up.sort();
        down.sort();

        Migrations {
            up,
            down,
            table_creation,
        }
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
}
