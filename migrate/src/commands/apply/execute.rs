use crate::{commands::ApplyCommand, Database};

impl ApplyCommand {
    /// Applies the required migrations to `database`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::open(self.database)?;
        db.migrate(&self.path)
    }
}
