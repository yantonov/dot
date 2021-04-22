use walkdir::DirEntry;
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;

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