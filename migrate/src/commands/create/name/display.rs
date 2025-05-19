use super::MigrationName;
use argparse::DefaultDisplay;

impl std::fmt::Display for MigrationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl DefaultDisplay for MigrationName {
    type Display<'a> = &'a Self;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self
    }
}
