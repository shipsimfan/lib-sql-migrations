use sql::{Column, FromRow, FromRowError};

/// A migration currently in the database
#[derive(Clone)]
pub struct DownMigration {
    /// The ID of the migration
    id: usize,

    /// The name of the migration
    name: String,
}

impl DownMigration {
    /// Gets the database id of the migration
    pub fn id(&self) -> usize {
        self.id
    }

    /// Gets the name of the migration
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for DownMigration {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for DownMigration {}

impl PartialOrd for DownMigration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DownMigration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl FromRow for DownMigration {
    fn from_row<'a, R: sql::Row<'a>>(row: R) -> Result<Self, R::Error> {
        let mut id = None;
        let mut name = None;

        for column in row {
            let column = column?;

            match column.name()?.as_str() {
                "id" => id = Some(column.into_usize()?),
                "name" => name = Some(column.into_str()?.to_string()),
                name => {
                    return Err(R::Error::custom(&format_args!(
                        "unknown column name \"{}\"",
                        name
                    )))
                }
            }
        }

        let id = id.ok_or(R::Error::missing_column("id"))?;
        let name = name.ok_or(R::Error::missing_column("name"))?;

        Ok(DownMigration { id, name })
    }
}
