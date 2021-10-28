use std::path::{Path, PathBuf};

use walkdir::DirEntry;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::util::to_result;

fn get_relative_file_name(root: &Path, entry: &DirEntry) -> Result<String, String> {
    let stripped = entry.path().strip_prefix(&root)
        .map_err(|_| "cannot strip prefix")?;
    to_result(stripped.to_str(), "cannot get file name")
        .map(|x| x.to_string())
}

pub fn target_path(context: &FileOperationContext, entry: &DirEntry) -> Result<PathBuf, String> {
    let file_name = get_relative_file_name(context.source_directory(), entry)?;
    Ok(Path::join(Path::new(&context.target_directory()), file_name))
}