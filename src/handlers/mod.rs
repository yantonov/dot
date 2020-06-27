use std::path::{Path, PathBuf};
use std::result::Result;

use walkdir::{DirEntry, WalkDir};

use crate::environment::Environment;

fn iterate_files<C>(root: &PathBuf,
                    context: &C,
                    file_operation: fn(&C, &DirEntry) -> Result<(), String>,
) -> Result<(), String> {
    for entry in WalkDir::new(root)
        .into_iter() {
        let e = entry
            .map_err(|e| e.to_string());

        if e.is_ok() {
            let entry_value = e.unwrap();
            if !entry_value.file_type().is_dir() {
                let result = file_operation(context, &entry_value);
                if result.is_err() {
                    println!("error={}", result.unwrap_err())
                }
            }
        }
    }
    Ok(())
}

struct FileOperationContext {
    home: String,
    current_directory: PathBuf,
}

fn create_file_operation_context(env: &Environment) -> Result<FileOperationContext, String> {
    let home = env.home_directory()?;
    let current_directory = env.current_dir()?;

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

fn link_file_operation(context: &FileOperationContext,
                       entry: &DirEntry) -> Result<(), String> {
    let file_name = get_relative_file_name(&context.current_directory, entry)?;

    let home_file_pathbuf = Path::join(Path::new(&context.home), file_name);
    let home_file_path = home_file_pathbuf.as_path();
    let repository_file_path = entry.path();
    if home_file_path.exists() {
        // TODO: if file exists and not equal to source file
        // create backup file and create symlink
    } else {
        let home_file_path_parent_dir = home_file_path.parent()
            .unwrap();
        if !home_file_path_parent_dir.exists() {
            std::fs::create_dir(home_file_path_parent_dir)
                .map_err(|e| e.to_string())?;
        }
        symlink::symlink_file(repository_file_path, &home_file_path)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn unlink_file_operation(context: &FileOperationContext,
                         entry: &DirEntry) -> Result<(), String> {
    let file_name = get_relative_file_name(&context.current_directory, entry)?;

    let home_file_pathbuf = Path::join(Path::new(&context.home), file_name);
    let home_file_path = home_file_pathbuf.as_path();
    let repository_file_path = entry.path();
    if home_file_path.exists() {
        if std::fs::read_link(home_file_path).is_ok() {
            std::fs::remove_file(home_file_path)
                .map_err(|e| e.to_string())?;
            std::fs::copy(repository_file_path, home_file_path)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn link(_environment: &Environment) -> Result<(), String> {
    let current_dir = _environment.current_dir()
        .map_err(|e| e.to_string())?;

    let context = create_file_operation_context(_environment)?;

    iterate_files(&current_dir,
                  &context,
                  link_file_operation)
}

pub fn unlink(_environment: &Environment) -> Result<(), String> {
    let current_dir = _environment.current_dir()
        .map_err(|e| e.to_string())?;

    let context = create_file_operation_context(_environment)?;

    iterate_files(&current_dir,
                  &context,
                  unlink_file_operation)
}
