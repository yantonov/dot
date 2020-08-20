use walkdir::DirEntry;

use crate::handlers::backup::lister::list_backup_files;
use crate::handlers::file_operation::FileOperation;
use crate::handlers::file_operation_context::FileOperationContext;

pub struct RemoveBackupOperation {}

impl FileOperation for RemoveBackupOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let files = list_backup_files(context, entry)?;
        for entry in files {
            let _ = std::fs::remove_file(entry.path());
        }
        Ok(())
    }
}