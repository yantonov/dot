use walkdir::DirEntry;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::home_path;
use std::path::Path;

pub struct CheckFileOperation {}

fn exists(home_path: &Path, repository_path: &Path) -> bool {
    if !home_path.exists() {
        return false;
    }
    let link = std::fs::read_link(home_path);
    return link.is_ok() && link.unwrap().as_path() == repository_path;
}

impl FileOperation for CheckFileOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        match exists(&home_path(context, &entry)?, entry.path()) {
            true => Ok(()),
            false => Err(format!("symlink does not exist"))
        }
    }
}
