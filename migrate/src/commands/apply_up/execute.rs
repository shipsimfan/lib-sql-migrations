use crate::{commands::ApplyUpCommand, Database};

impl ApplyUpCommand {
    /// Applies the required up migrations to `database`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::open(self.database)?;

        let mut migrations = db.get_required_migrations(&self.path)?;
        migrations.clear_down();
        db.apply_migrations(&migrations)
    }
}
