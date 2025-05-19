use crate::{commands::RequiredUpCommand, Database};

impl RequiredUpCommand {
    /// Displays the up migrations that must be applied to `database` from `path`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::open(self.database)?;

        let migrations = db.get_required_migrations(&self.path)?;
        for migration in migrations.up() {
            println!("{}", migration.name());
        }

        Ok(())
    }
}
