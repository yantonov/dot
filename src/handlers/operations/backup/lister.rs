use walkdir::{DirEntry, WalkDir};

use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::{home_path};
use crate::handlers::operations::backup::name_convention::is_backup_file;

pub fn list_backup_files(context: &FileOperationContext,
                         entry: &DirEntry) -> Result<Vec<DirEntry>, String> {
    let file_name = entry.file_name().to_str().unwrap();
    let home_file_pathbuf = home_path(context, &entry)?;
    let home_file_path = home_file_pathbuf.as_path();
    let file_directory = home_file_path.parent().unwrap();
    let backup_checker = is_backup_file(&file_name);
    Ok(
        WalkDir::new(file_directory)
            .max_depth(1)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
            .filter(|entry| entry.is_ok())
            .map(|entry| entry.unwrap())
            .filter(|entry|
                backup_checker(entry.file_name().to_str().unwrap())
            )
            .into_iter()
            .collect()
    )
}