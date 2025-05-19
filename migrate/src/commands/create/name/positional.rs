use super::{InvalidMigrationName, MigrationName};
use argparse::{Argument, Error, Positional, PositionalInfo, PositionalResult};

impl Positional for MigrationName {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut new = None;
        String::parse(&mut new, argument, &info.drop_default())?;
        let name = new.unwrap();

        if name.len() == 0 {
            return PositionalResult::Error(Error::invalid_positional_value(
                info.value,
                InvalidMigrationName::new(name),
            ));
        }

        for c in name.chars() {
            if !c.is_ascii_lowercase() && !c.is_digit(10) && c != '-' {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidMigrationName::new(name),
                ));
            }
        }

        *this = Some(MigrationName { name });

        PositionalResult::Next
    }
}
