use crate::{commands::ApplyDownCommand, Database};

impl ApplyDownCommand {
    /// Applies the required down migrations to `database`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::open(self.database)?;

        let mut migrations = db.get_required_migrations(&self.path)?;
        migrations.clear_up();
        db.apply_migrations(&migrations)
    }
}
