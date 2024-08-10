use validate_name::is_valid_name;

mod down;
mod up;
mod validate_name;

pub use down::DownMigration;
pub use up::UpMigration;

const DOWN_SUFFIX: &str = ".down.sql";
const UP_SUFFIX: &str = ".up.sql";
