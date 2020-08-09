use std::path::{Path, PathBuf};
use std::result::Result;

use chrono::Local;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use crate::environment::Environment;
use crate::log::{Logger, LogLevel};

fn iterate_files<C>(root: &PathBuf,
                    context: &C,
                    file_operation: &dyn Fn(&C, &DirEntry) -> Result<(), String>,
) -> Result<(), String> {
    for entry in WalkDir::new(root)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter() {
        let e = entry
            .map_err(|e| e.to_string());

        if e.is_ok() {
            let entry_value = e.unwrap();
            if !entry_value.file_type().is_dir() {
                let _ = file_operation(context, &entry_value);
            }
        }
    }
    Ok(())
}

struct FileOperationContext<'a> {
    home: &'a String,
    current_directory: &'a PathBuf,
}

fn create_file_operation_context(env: &Environment) -> Result<FileOperationContext, String> {
    let home = env.home_directory();
    let current_directory = env.current_dir();

    Ok(
        FileOperationContext {
            home,
            current_directory,
        })
}

fn get_relative_file_name(root: &Path, entry: &DirEntry) -> Result<String, String> {
    let stripped = entry.path().strip_prefix(&root);
    if stripped.is_err() {
        return Err("cannot strip prefix".to_string());
    }
    let option = stripped.unwrap().to_str();
    match option {
        None => Err("cannot extract file name for file".to_string()),
        Some(v) => Ok(v.to_string()),
    }
}

fn get_timestamp_string() -> String {
    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
}

fn create_backup_file(home_file_path: &Path,
                      repository_path: &Path) -> Result<(), String> {
    if !home_file_path.exists() {
        return Ok(());
    }
    let link = std::fs::read_link(home_file_path);
    if link.is_ok() && link.unwrap().as_path() == repository_path {
        return Ok(());
    }

    let s: String = vec![
        home_file_path.to_str().unwrap(),
        ".bak.",
        &get_timestamp_string()
    ]
        .join("");
    let backup_file_copy_path = Path::new(
        &s);

    std::fs::copy(home_file_path, backup_file_copy_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn create_parent_directory(home_file_path: &Path) -> Result<(), String> {
    let home_file_path_parent_dir = home_file_path.parent()
        .unwrap();
    if !home_file_path_parent_dir.exists() {
        std::fs::create_dir_all(home_file_path_parent_dir)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn link_file_operation(context: &FileOperationContext,
                       entry: &DirEntry) -> Result<(), String> {
    let file_name = get_relative_file_name(&context.current_directory, entry)?;

    let home_file_pathbuf = Path::join(Path::new(&context.home), file_name);
    let home_file_path = home_file_pathbuf.as_path();
    let repository_file_path = entry.path();

    create_parent_directory(&home_file_path)?;

    create_backup_file(&home_file_path, &repository_file_path)?;

    if home_file_path.exists() {
        std::fs::remove_file(home_file_path)
            .map_err(|e| e.to_string())?;
    }
    symlink::symlink_file(repository_file_path, &home_file_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn unlink_file_operation(context: &FileOperationContext,
                         entry: &DirEntry) -> Result<(), String> {
    let file_name = get_relative_file_name(&context.current_directory, entry)?;

    let home_file_pathbuf = Path::join(Path::new(&context.home), file_name);
    let home_file_path = home_file_pathbuf.as_path();
    let repository_file_path = entry.path();
    if home_file_path.exists() {
        let link = std::fs::read_link(home_file_path);
        if link.is_ok() {
            if link.unwrap().as_path() == entry.path() {
                std::fs::remove_file(home_file_path)
                    .map_err(|e| e.to_string())?;
                std::fs::copy(repository_file_path, home_file_path)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

fn list_file_operation(_: &FileOperationContext,
                       entry: &DirEntry) -> Result<(), String> {
    let repository_file_path = entry.path();
    if let Some(value) = repository_file_path.to_str() {
        println!("{}", value);
    }
    Ok(())
}

fn is_backup_file(file_name: &str) -> bool {
    let re = Regex::new(r"(^.*\.bak\.\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}$)").unwrap();
    re.is_match(file_name)
}

fn list_backup_operation(context: &FileOperationContext,
                         entry: &DirEntry) -> Result<(), String> {
    let file_name = get_relative_file_name(&context.current_directory, entry)?;

    let home_file_pathbuf = Path::join(Path::new(&context.home), file_name);
    let home_file_path = home_file_pathbuf.as_path();
    let home_file_directory = home_file_path.parent().unwrap();

    for entry in WalkDir::new(home_file_directory)
        .max_depth(1)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter() {
        let e = entry
            .map_err(|e| e.to_string());

        if e.is_ok() {
            let entry_value = e.unwrap();
            if !entry_value.file_type().is_dir() {
                let file_name = entry_value.file_name().to_str().unwrap();
                if is_backup_file(file_name) {
                    let file_path = entry_value.path().to_str().unwrap();
                    println!("{}", file_path);
                }
            }
        }
    }

    Ok(())
}

fn wrap_with_log<'a, C>(_logger: &'a Logger,
                        operation: &'a (dyn Fn(&C, &DirEntry) -> Result<(), String> )) ->
                        impl Fn(&C, &DirEntry) -> Result<(), String> + 'a {
    move |context: &C, entry_value: &DirEntry| {
        let result = operation(context, &entry_value);
        let entry_path_str = entry_value.path().to_str().unwrap();
        if result.is_err() {
            _logger.log(LogLevel::Error,
                        &format!("{} - {}",
                                 entry_path_str,
                                 result.unwrap_err()))
        } else {
            _logger.log(LogLevel::Info,
                        &format!("{}",
                                 entry_path_str))
        }
        Ok(())
    }
}

pub fn link(_environment: &Environment,
            _logger: &Logger) -> Result<(), String> {
    let current_dir = _environment.current_dir();

    let context = create_file_operation_context(_environment)?;

    iterate_files(&current_dir,
                  &context,
                  &wrap_with_log(_logger, &link_file_operation))
}

pub fn unlink(_environment: &Environment,
              _logger: &Logger) -> Result<(), String> {
    let current_dir = _environment.current_dir();

    let context = create_file_operation_context(_environment)?;

    iterate_files(&current_dir,
                  &context,
                  &wrap_with_log(_logger, &unlink_file_operation))
}

pub fn list(_environment: &Environment,
            _: &Logger) -> Result<(), String> {
    let current_dir = _environment.current_dir();

    let context = create_file_operation_context(_environment)?;

    iterate_files(&current_dir,
                  &context,
                  &list_file_operation)
}

pub fn list_backup(_environment: &Environment,
                   _: &Logger) -> Result<(), String> {
    let current_dir = _environment.current_dir();

    let context = create_file_operation_context(_environment)?;

    iterate_files(&current_dir,
                  &context,
                  &list_backup_operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_file_pattern_test() {
        assert_eq!(true, is_backup_file("test.bak.2020-01-01_12-01-01"));
        assert_eq!(true, is_backup_file("test.bak.bak.2020-01-01_12-01-01"));
    }

    #[test]
    fn not_backup_file_pattern_test() {
        assert_eq!(false, is_backup_file("test.txt"));
        assert_eq!(false, is_backup_file("test.txt.bak"));
        assert_eq!(false, is_backup_file("test.txt.bak.2020-01-01"));
    }
}
