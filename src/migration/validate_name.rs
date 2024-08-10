const MONTH_LENGTHS: [usize; 12] = [31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const FEBRUARY_SHORT: usize = 28;
const FEBRUARY_LONG: usize = 29;

/// Checks if `name` only contains digits, lowercase letters, and dashes and starts with a valid
/// date and time in the format `YYYYMMDD-HHmm-`
pub(super) fn is_valid_name(name: &str) -> bool {
    if !starts_with_datetime(name) {
        return false;
    }

    for c in name.chars() {
        if !c.is_alphanumeric() && c != '-' {
            return false;
        }
    }

    true
}

/// Checks if `name` starts with a date and time in format `YYYYMMDD-HHmm-`
///
/// This function does some validation on the date and time but it does not check fully for leap
/// years and will let February 29th exist in all years divisible by 4 even when not valid. For
/// example, this function will accept the date `19000229` despite not being a valid date.
fn starts_with_datetime(name: &str) -> bool {
    if name.len() < 14 {
        return false;
    }

    let name = name.as_bytes();

    // Check year
    let year = match check_number(&name[..4]) {
        None => return false,
        Some(year) => year,
    };

    // Check month
    let month = match check_number(&name[4..6]) {
        None | Some(0) => return false,
        Some(x) if x > 12 => return false,
        Some(month) => month,
    };

    // Check day
    let max_day = if month == 2 {
        if year % 4 == 0 {
            FEBRUARY_LONG
        } else {
            FEBRUARY_SHORT
        }
    } else {
        MONTH_LENGTHS[month - 1]
    };
    match check_number(&name[6..8]) {
        None | Some(0) => return false,
        Some(x) if x > max_day => return false,
        Some(_) => {}
    }

    // Check first dash
    if name[8] != b'-' {
        return false;
    }

    // Check hour
    match check_number(&name[9..11]) {
        None => return false,
        Some(x) if x > 23 => return false,
        Some(_) => {}
    }

    // Check minute
    match check_number(&name[11..13]) {
        None => return false,
        Some(x) if x > 59 => return false,
        Some(_) => {}
    }

    // Check last dash
    if name[13] != b'-' {
        return false;
    }

    true
}

/// Validates that `str` is a number and returns it
fn check_number(str: &[u8]) -> Option<usize> {
    let mut value = 0;
    for c in str {
        if !c.is_ascii_digit() {
            return None;
        }

        value *= 10;
        value += (c - b'0') as usize;
    }
    Some(value)
}
