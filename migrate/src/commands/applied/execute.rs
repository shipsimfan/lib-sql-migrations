use crate::{commands::AppliedCommand, Database};

impl AppliedCommand {
    /// Lists the applied migrations to `database`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::open(self.database)?;

        let migrations = match db.get_applied_migrations()? {
            Some(migrations) => migrations,
            None => return Ok(println!("\"applied_migration\" table not created")),
        };

        for migration in migrations {
            println!("{}", migration.name());
        }

        Ok(())
    }
}
