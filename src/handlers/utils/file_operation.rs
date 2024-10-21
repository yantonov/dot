use std::path::Path;

use walkdir::{DirEntry, WalkDir};
use crate::handlers::utils::file_operation_context::FileOperationContext;

pub trait FileOperation {
    fn call(&self, context: &FileOperationContext<'_>, entry: &DirEntry) -> Result<(), String>;
}

pub fn iterate_files(root: &Path,
                     context: &FileOperationContext<'_>,
                     file_operation: &dyn FileOperation,
) -> Result<(), String> {
    WalkDir::new(root)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| !entry.file_type().is_dir())
        .fold(Ok(()), |result, entry| {
            let operation_result = file_operation.call(context, &entry);
            match operation_result {
                Ok(_) => result,
                Err(_) => operation_result
            }
        })
}