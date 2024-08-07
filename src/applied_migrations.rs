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
