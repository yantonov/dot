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
    // every entry must be processed even after an earlier one fails, so each
    // failing file gets reported instead of stopping at the first error -
    // try_fold's short-circuit-on-Err behavior would be wrong here.
    #[allow(clippy::manual_try_fold)]
    let result = WalkDir::new(root)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.file_type().is_dir())
        .fold(Ok(()), |result, entry| {
            let operation_result = file_operation.call(context, &entry);
            match operation_result {
                Ok(_) => result,
                Err(_) => operation_result
            }
        });
    result
}