use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

pub trait FileOperation {
    type Context;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String>;
}

pub fn iterate_files<TContext>(root: &PathBuf,
                               context: &TContext,
                               file_operation: &dyn FileOperation<Context=TContext>,
) {
    for entry in WalkDir::new(root)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| !entry.file_type().is_dir()) {
        let _ = file_operation.call(context, &entry);
    }
}