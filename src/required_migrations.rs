use crate::ApplyMigrationError;
use sql::{Connection, Statement};
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

/// The migrations the need to applied to the database
pub(crate) struct RequiredMigrations {
    /// Does the applied migrations table need to be created?
    table_creation_required: bool,

    /// The migrations which must be undone
    down: Vec<PathBuf>,

    /// The migrations which must be newly applied
    up: Vec<PathBuf>,
}

impl RequiredMigrations {
    /// Gets the migrations that need to be applied to the database in the order they need to be
    /// applied
    ///
    /// Dependencies aren't explicitly stated in the migrations, instead their order is based on the
    /// dates baked into their filenames. The implied dependency is that later "down" scripts must be
    /// applied before earlier ones and later "up" scripts must be applied after earlier ones.
    pub(crate) fn get<C: Connection>(
        connection: &C,
        path: &Path,
    ) -> Result<RequiredMigrations, ApplyMigrationError> {
        let available_migrations = get_available_migrations(path).map_err(|error| {
            ApplyMigrationError::GetAvailableMigrationsFailed(error, path.to_path_buf())
        })?;

        // Get the current list of applied migrations
        let applied_migrations = get_applied_migrations(connection)
            .map_err(ApplyMigrationError::GetAppliedMigrationsFailed)?;
        let table_creation_required = applied_migrations.is_none();
        let applied_migrations = applied_migrations.unwrap_or(Vec::new());

        // Diff the available migrations against the applied migrations
        let mut required_migrations = diff_migrations(
            path,
            table_creation_required,
            available_migrations,
            applied_migrations,
        );

        // Sort the resulting lists
        required_migrations.up.sort();
        required_migrations.down.sort_by(|a, b| a.cmp(b).reverse());

        Ok(required_migrations)
    }

    /// Gets the down migrations which need to be applied
    pub(crate) fn down(&self) -> &[PathBuf] {
        &self.down
    }

    /// Gets the up migrations which need to be applied
    pub(crate) fn up(&self) -> &[PathBuf] {
        &self.up
    }

    /// Does the applied migrations table need to be created?
    pub(crate) fn table_creation_required(&self) -> bool {
        self.table_creation_required
    }
}

/// Gets the list of available migrations by inspecting the [`MIGRATIONS_DIR`] folder for files
/// which end with ".up.sql"
fn get_available_migrations(directory: &Path) -> std::io::Result<Vec<(PathBuf, String)>> {
    let dir = match std::fs::read_dir(directory) {
        Ok(dir) => dir,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut available_migrations = Vec::new();
    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            _ => continue,
        };

        let is_dir = entry
            .file_type()
            .map(|file_type| file_type.is_dir())
            .unwrap_or(false);
        if is_dir {
            let sub_migrations = get_available_migrations(&entry.path())?;
            available_migrations.extend(sub_migrations);
            continue;
        }

        if let Some(migration_name) = entry
            .file_name()
            .as_encoded_bytes()
            .strip_suffix(b".up.sql")
        {
            if migration_name.len() == 0 {
                continue;
            }

            available_migrations.push((
                entry.path(),
                String::from_utf8_lossy(migration_name).to_string(),
            ));
        }
    }

    Ok(available_migrations)
}

/// Gets the list of migrations which have been applied to the database
fn get_applied_migrations<C: Connection>(connection: &C) -> Result<Option<Vec<String>>, String> {
    // Check if the table exists
    let mut rows = connection
        .prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='applied_migration';",
        )
        .map_err(|error| error.to_string())?
        .rows()
        .map_err(|error| error.to_string())?;

    let count = rows.next().map(|row| row.unwrap_or(0)).unwrap_or(0);
    if count == 0 {
        return Ok(None);
    }

    // Read the columns from the table
    let mut select = connection
        .prepare("SELECT name FROM applied_migration")
        .map_err(|error| error.to_string())?
        .rows()
        .map_err(|error| error.to_string())?;
    let mut applied_migrations = Vec::new();
    while let Some(row) = select.next() {
        applied_migrations.push(row.map_err(|error| error.to_string())?);
    }

    Ok(Some(applied_migrations))
}

/// Gets the required migrations from the difference between the available and applied migrations
fn diff_migrations(
    base_path: &Path,
    table_creation_required: bool,
    mut available_migrations: Vec<(PathBuf, String)>,
    mut applied_migrations: Vec<String>,
) -> RequiredMigrations {
    let mut i = 0;
    while i < available_migrations.len() {
        let mut found = false;
        for j in 0..applied_migrations.len() {
            if applied_migrations[j] == available_migrations[i].1 {
                found = true;
                applied_migrations.swap_remove(j);
                break;
            }
        }

        if found {
            available_migrations.swap_remove(i);
        } else {
            i += 1;
        }
    }

    RequiredMigrations {
        table_creation_required,
        down: applied_migrations
            .into_iter()
            .map(|migration| Path::new(base_path).join(format!("{}.down.sql", migration)))
            .collect(),
        up: available_migrations
            .into_iter()
            .map(|migration| migration.0)
            .collect(),
    }
}
