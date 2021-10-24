use std::path::{Path, PathBuf};

use chrono::Local;
use regex::Regex;
use crate::util::to_result;

pub fn is_backup_file(original_file: &str) -> impl Fn(&str) -> bool {
    let string = format!("^{}\\.bak\\.\\d{{4}}-\\d{{2}}-\\d{{2}}_\\d{{2}}-\\d{{2}}-\\d{{2}}$",
                         regex::escape(original_file));
    let re = Regex::new(&string).unwrap();
    move |file_to_test| re.is_match(file_to_test)
}

fn get_timestamp_string() -> String {
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}

pub fn get_backup_file_path(file_path: &Path) -> Result<PathBuf, String> {
    let path_str: String = vec![
        to_result(file_path.to_str(), "cannot get file name")?,
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
        assert_eq!(true, is_backup_file("test")
            ("test.bak.2020-01-01_12-01-01"));
        assert_eq!(true, is_backup_file("test.bak")
            ("test.bak.bak.2020-01-01_12-01-01"));
    }

    #[test]
    fn not_backup_file_pattern_test() {
        assert_eq!(false, is_backup_file("test.txt")
            ("test.txt"));
        assert_eq!(false, is_backup_file("test.txt")
            ("test.txt.bak"));
        assert_eq!(false, is_backup_file("test.txt")
            ("test.txt.bak.2020-01-01"));
        assert_eq!(false, is_backup_file("prefix")
            ("prefix_test.txt.bak.2020-01-01_12-01-01"));
    }
}