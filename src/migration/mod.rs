use validate_name::is_valid_name;

mod down;
mod up;
mod validate_name;

pub use down::DownMigration;
pub use up::UpMigration;

/// The ending for down migration file names
pub const DOWN_SUFFIX: &str = ".down.sql";

/// The ending for up migration file names
pub const UP_SUFFIX: &str = ".up.sql";
