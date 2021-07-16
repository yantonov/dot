use walkdir::DirEntry;

use crate::handlers::operations::backup::lister::list_backup_files;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;

pub struct ListBackupOperation {}

impl FileOperation for ListBackupOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let files = list_backup_files(context, entry)?;
        for entry in files {
            if let Some(filename) = entry.path().to_str() {
                println!("{}", filename);
            }
        }
        Ok(())
    }
}