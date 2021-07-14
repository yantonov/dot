use std::path::{Path, PathBuf};

use walkdir::DirEntry;
use crate::handlers::utils::file_operation_context::FileOperationContext;

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

pub fn home_path(context: &FileOperationContext, entry: &DirEntry) -> Result<PathBuf, String> {
    let file_name = get_relative_file_name(&context.current_directory(), entry)?;
    Ok(Path::join(Path::new(&context.home()), file_name))
}