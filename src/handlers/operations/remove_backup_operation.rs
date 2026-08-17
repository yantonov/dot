use walkdir::DirEntry;

use crate::handlers::operations::backup::lister::list_backup_files;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;

pub struct RemoveBackupOperation {}

impl FileOperation for RemoveBackupOperation {

    fn call(&self, context: &FileOperationContext, entry: &DirEntry) -> Result<(), String> {
        let files = list_backup_files(context, entry)?;
        for entry in files {
            if context.dry_run() {
                context.logger().log_dry_run_plan(&format!(
                    "would remove backup file {}",
                    entry.path().display()));
                continue;
            }
            std::fs::remove_file(entry.path())
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}