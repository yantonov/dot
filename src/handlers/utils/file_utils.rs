use std::path::{Path, PathBuf};

use walkdir::DirEntry;
use crate::handlers::utils::file_operation_context::FileOperationContext;

fn get_relative_file_name(root: &Path, entry: &DirEntry) -> Result<String, String> {
    let stripped = entry.path().strip_prefix(&root)
        .map_err(|_| "cannot strip prefix")?;
    stripped.to_str().ok_or("cannot get file name").map(|x| x.to_string())
}

pub fn target_path(context: &FileOperationContext, entry: &DirEntry) -> Result<PathBuf, String> {
    let file_name = get_relative_file_name(context.source_directory(), entry)?;
    Ok(context.target_directory().join(file_name))
}