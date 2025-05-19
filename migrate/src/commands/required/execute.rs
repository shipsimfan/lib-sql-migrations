use crate::{commands::RequiredCommand, Database};

impl RequiredCommand {
    /// Displays the migrations that must be applied to `database` from `path`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::open(self.database)?;

        let migrations = db.get_required_migrations(&self.path)?;

        if migrations.down().len() > 0 {
            println!("Down migrations:");
            for migration in migrations.down() {
                println!("  {}", migration.name());
            }
        }

        if migrations.up().len() > 0 {
            if migrations.down().len() > 0 {
                println!();
            }

            println!("Up migrations:");
            for migration in migrations.up() {
                println!("  {}", migration.name());
            }
        }

        if migrations.down().len() == 0 && migrations.up().len() == 0 {
            println!("No migrations required");
        }

        Ok(())
    }
}
