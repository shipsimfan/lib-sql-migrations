use std::path::Path;

/// Lists the available migrations in `path`
pub fn available(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let migrations = sql_migrations::get_available_migrations(path)?;
    for migration in migrations {
        println!("{}", migration.name());
    }

    Ok(())
}
