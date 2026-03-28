use std::path::{Path, PathBuf};

use chrono::Local;

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
        file_to_test.strip_prefix(prefix.as_str())
            .is_some_and(is_timestamp)
    }
}

fn get_timestamp_string() -> String {
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}

pub fn get_backup_file_path(file_path: &Path) -> Result<PathBuf, String> {
    let path_str: String = vec![
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
        assert!(!is_timestamp("2024-03-28_14-30-4"));  // one char short
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

    #[test]
    fn backup_file_pattern_test() {
        assert!(is_backup_file("test")("test.bak.2020-01-01_12-01-01"));
        assert!(is_backup_file("test.bak")("test.bak.bak.2020-01-01_12-01-01"));
    }

    #[test]
    fn not_backup_file_pattern_test() {
        assert!(!is_backup_file("test.txt")("test.txt"));
        assert!(!is_backup_file("test.txt")("test.txt.bak"));
        assert!(!is_backup_file("test.txt")("test.txt.bak.2020-01-01"));
        assert!(!is_backup_file("prefix")("prefix_test.txt.bak.2020-01-01_12-01-01"));
    }
}