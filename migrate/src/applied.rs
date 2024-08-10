use crate::{database::OpenDatabase, Database};

/// Lists the applied migrations to `database`
pub fn applied(database: Option<&Database>) -> Result<(), Box<dyn std::error::Error>> {
    let db = OpenDatabase::open(database)?;

    let migrations = match db.get_applied_migrations()? {
        Some(migrations) => migrations,
        None => return Ok(println!("\"applied_migration\" table not created")),
    };

    for migration in migrations {
        println!("{}", migration.name());
    }

    Ok(())
}
