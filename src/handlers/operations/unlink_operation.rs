use std::path::Path;

use walkdir::DirEntry;

use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::file_utils::target_path;

pub struct UnlinkFileOperation {}

impl UnlinkFileOperation {
    fn is_linked_to_source(&self, target_file_path: &Path, source_file_path: &Path) -> bool {
        target_file_path.exists()
            && std::fs::read_link(target_file_path)
                .is_ok_and(|link| link.as_path() == source_file_path)
    }
}

impl FileOperation for UnlinkFileOperation {
    fn call(&self, context: &FileOperationContext, entry: &DirEntry) -> Result<(), String> {
        let target_file_pathbuf = target_path(context, entry)?;
        let target_file_path = target_file_pathbuf.as_path();
        let source_file_path = entry.path();

        if !self.is_linked_to_source(target_file_path, source_file_path) {
            return Ok(());
        }

        if context.dry_run() {
            context.logger().log_dry_run_plan(&format!(
                "would replace symlink {} with a regular copy of {}",
                target_file_path.display(),
                source_file_path.display()
            ));
            return Ok(());
        }

        std::fs::remove_file(target_file_path).map_err(|e| e.to_string())?;
        std::fs::copy(source_file_path, target_file_path).map_err(|e| e.to_string())?;
        Ok(())
    }
}
