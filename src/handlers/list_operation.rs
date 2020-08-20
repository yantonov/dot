use crate::handlers::file_operation::FileOperation;
use crate::handlers::file_operation_context::FileOperationContext;
use walkdir::DirEntry;

pub struct ListFileOperation {}

impl FileOperation for ListFileOperation {
    type Context = FileOperationContext;

    fn call(&self, _: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let repository_file_path = entry.path();
        if let Some(value) = repository_file_path.to_str() {
            println!("{}", value);
        }
        Ok(())
    }
}