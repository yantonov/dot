use walkdir::DirEntry;

use crate::handlers::backup::lister::list_backup_files;
use crate::handlers::file_operation::FileOperation;
use crate::handlers::file_operation_context::FileOperationContext;

pub struct ListBackupOperation {}

impl FileOperation for ListBackupOperation {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let files = list_backup_files(context, entry)?;
        for entry in files {
            println!("{}", entry.path().to_str().unwrap());
        }
        Ok(())
    }
}