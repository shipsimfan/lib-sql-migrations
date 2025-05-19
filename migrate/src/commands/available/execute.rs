use crate::commands::AvailableCommand;

impl AvailableCommand {
    /// Displays the available migrations in `path`
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let migrations = sql_migrations::get_available_migrations(&self.path)?;
        for migration in migrations {
            println!("{}", migration.name());
        }

        Ok(())
    }
}
