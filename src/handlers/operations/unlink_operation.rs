use walkdir::DirEntry;

use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::{target_path};

pub struct UnlinkFileOperation {}

impl FileOperation for UnlinkFileOperation {

    fn call(&self, context: &FileOperationContext, entry: &DirEntry) -> Result<(), String> {
        let target_file_pathbuf = target_path(context, entry)?;
        let target_file_path = target_file_pathbuf.as_path();
        let source_file_path = entry.path();
        if target_file_path.exists() {
            if let Ok(link) = std::fs::read_link(target_file_path) {
                if link.as_path() == entry.path() {
                    std::fs::remove_file(target_file_path)
                        .map_err(|e| e.to_string())?;
                    std::fs::copy(source_file_path, target_file_path)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }
}