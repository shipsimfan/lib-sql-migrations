use super::{is_valid_name, DOWN_SUFFIX, UP_SUFFIX};
use std::path::{Path, PathBuf};

/// A migration ready to be applied
#[derive(Clone)]
pub struct UpMigration {
    /// The path to the migration SQL
    source: PathBuf,

    /// The name of the migration
    name: String,
}

impl UpMigration {
    /// Checks if `source` is a valid migration
    pub(crate) fn from_path(source: PathBuf) -> Option<Self> {
        let name = source
            .file_name()?
            .to_str()?
            .strip_suffix(UP_SUFFIX)?
            .to_string();

        if !is_valid_name(&name) {
            return None;
        }

        Some(UpMigration { source, name })
    }

    /// Gets the path to the up migration SQL
    pub fn up_source(&self) -> &Path {
        &self.source
    }

    /// Gets the path to the down migration SQL
    pub fn down_source(&self, base_path: &Path) -> PathBuf {
        base_path.join(format!("{}{}", self.name, DOWN_SUFFIX))
    }

    /// Gets the name of the migration
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for UpMigration {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for UpMigration {}

impl PartialOrd for UpMigration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UpMigration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
