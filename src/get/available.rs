use crate::{MigrationError, UpMigration};
use std::path::Path;

/// Gets the available migrations in `path`
pub fn get_available_migrations(path: &Path) -> Result<Vec<UpMigration>, MigrationError> {
    let mut migrations = Vec::new();
    for entry in std::fs::read_dir(path)
        .map_err(|error| MigrationError::OpenMigrationsFailed(error, path.to_owned()))?
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if !entry
            .file_type()
            .map(|file_type| file_type.is_file())
            .unwrap_or(false)
        {
            continue;
        }

        if let Some(migration) = UpMigration::from_path(entry.path()) {
            migrations.push(migration);
        }
    }

    Ok(migrations)
}
