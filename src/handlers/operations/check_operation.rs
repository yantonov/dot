use walkdir::DirEntry;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::target_path;
use std::path::Path;

pub struct CheckFileOperation {}

fn exists(target_path: &Path,
          source_path: &Path) -> bool {
    if !target_path.exists() {
        return false;
    }
    let link = std::fs::read_link(target_path);
    link.is_ok() && link.unwrap().as_path() == source_path
}

impl FileOperation for CheckFileOperation {
    fn call(&self, context: &FileOperationContext<'_>, entry: &DirEntry) -> Result<(), String> {
        match exists(&target_path(context, entry)?, entry.path()) {
            true => Ok(()),
            false => Err("symlink does not exist".to_string())
        }
    }
}
