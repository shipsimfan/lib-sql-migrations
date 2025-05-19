use super::InvalidMigrationName;

impl std::fmt::Display for InvalidMigrationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid name \"{}\", name must only contain lowercase letters, numbers, and dashes",
            self.name
        )
    }
}
