use walkdir::DirEntry;

use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::{home_path};

pub struct UnlinkFileOperation {}

impl FileOperation for UnlinkFileOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let home_file_pathbuf = home_path(context, &entry)?;
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
}