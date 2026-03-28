use std::path::{Path, PathBuf};

use chrono::Local;

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