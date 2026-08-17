use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// Validates a timestamp string in the format: YYYY-MM-DD_HH-MM-SS
// Example: 2024-03-28_14-30-45
// Positions: 0123-56-89_12-45-78
//   [0..3]  year  (digits)
//   [4]     '-'
//   [5..6]  month (digits)
//   [7]     '-'
//   [8..9]  day   (digits)
//   [10]    '_'
//   [11..12] hour (digits)
//   [13]    '-'
//   [14..15] minute (digits)
//   [16]    '-'
//   [17..18] second (digits)
fn is_timestamp(s: &str) -> bool {
    s.len() == 19
        && s.chars().enumerate().all(|(i, c)| match i {
            4 | 7 => c == '-',
            10 => c == '_',
            13 | 16 => c == '-',
            _ => c.is_ascii_digit(),
        })
}

pub fn is_backup_file(original_file: &str) -> impl Fn(&str) -> bool {
    let prefix = format!("{}.bak.", original_file);
    move |file_to_test| {
        file_to_test
            .strip_prefix(prefix.as_str())
            .is_some_and(is_timestamp)
    }
}

// Converts a day count since 1970-01-01 (UTC) into a (year, month, day)
// civil calendar date. Proleptic Gregorian algorithm from Howard Hinnant's
// public-domain civil_from_days:
// http://howardhinnant.github.io/date_algorithms.html#civil_from_days
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

// UTC rather than local time: avoids pulling in a timezone-aware crate for a
// value that only needs to be unique and human-readable.
fn get_timestamp_string() -> String {
    let total_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let days = total_secs.div_euclid(86400);
    let secs_of_day = total_secs.rem_euclid(86400);
    let (year, month, day) = civil_from_days(days);
    let hour = secs_of_day / 3600;
    let minute = (secs_of_day % 3600) / 60;
    let second = secs_of_day % 60;
    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        year, month, day, hour, minute, second
    )
}

pub fn get_backup_file_path(file_path: &Path) -> Result<PathBuf, String> {
    let path_str: String = [
        file_path.to_str().ok_or("cannot get file name")?,
        ".bak.",
        &get_timestamp_string(),
    ]
    .join("");
    Ok(Path::new(&path_str).to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Timestamp format: YYYY-MM-DD_HH-MM-SS
    #[test]
    fn timestamp_valid() {
        assert!(is_timestamp("2024-03-28_14-30-45"));
        assert!(is_timestamp("2000-01-01_00-00-00")); // boundary: all zeros
        assert!(is_timestamp("9999-12-31_23-59-59")); // boundary: all max digits
    }

    #[test]
    fn timestamp_wrong_length() {
        assert!(!is_timestamp(""));
        assert!(!is_timestamp("2024-03-28_14-30-4")); // one char short
        assert!(!is_timestamp("2024-03-28_14-30-450")); // one char long
    }

    #[test]
    fn timestamp_wrong_separators() {
        assert!(!is_timestamp("2024_03-28_14-30-45")); // '_' instead of '-' at position 4
        assert!(!is_timestamp("2024-03-28-14-30-45")); // '-' instead of '_' at position 10
        assert!(!is_timestamp("2024-03-28_14:30:45")); // ':' instead of '-' at positions 13, 16
    }

    #[test]
    fn timestamp_non_digit_where_digit_expected() {
        assert!(!is_timestamp("202X-03-28_14-30-45")); // letter in year
        assert!(!is_timestamp("2024-0X-28_14-30-45")); // letter in month
        assert!(!is_timestamp("2024-03-2X_14-30-45")); // letter in day
    }

    // Reference day-counts cross-checked independently against `date -u -d`,
    // not derived from the algorithm under test.
    #[test]
    fn civil_from_days_reference_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(1), (1970, 1, 2));
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
        assert_eq!(civil_from_days(19810), (2024, 3, 28));
        assert_eq!(civil_from_days(11016), (2000, 2, 29)); // leap day, div by 400
        assert_eq!(civil_from_days(10956), (1999, 12, 31)); // day before that leap year
        assert_eq!(civil_from_days(-25509), (1900, 2, 28)); // not a leap year (div by 100, not 400)
        assert_eq!(civil_from_days(-25508), (1900, 3, 1)); // no Feb 29 in between
        assert_eq!(civil_from_days(19722), (2023, 12, 31));
        assert_eq!(civil_from_days(19723), (2024, 1, 1));
    }

    #[test]
    fn backup_file_pattern_test() {
        assert!(is_backup_file("test")("test.bak.2020-01-01_12-01-01"));
        assert!(is_backup_file("test.bak")(
            "test.bak.bak.2020-01-01_12-01-01"
        ));
    }

    #[test]
    fn not_backup_file_pattern_test() {
        assert!(!is_backup_file("test.txt")("test.txt"));
        assert!(!is_backup_file("test.txt")("test.txt.bak"));
        assert!(!is_backup_file("test.txt")("test.txt.bak.2020-01-01"));
        assert!(!is_backup_file("prefix")(
            "prefix_test.txt.bak.2020-01-01_12-01-01"
        ));
    }
}
