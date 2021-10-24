use std::path::PathBuf;

use crate::environment::Environment;

pub struct FileOperationContext {
    target_directory: PathBuf,
    source_directory: PathBuf,
}

impl FileOperationContext {
    pub fn create(env: &Environment) -> FileOperationContext {
        let target_directory = env.target_directory().clone();
        let source_directory = env.source_directory().clone();

        FileOperationContext {
            target_directory,
            source_directory,
        }
    }

    pub fn target_directory(&self) -> &PathBuf {
        &self.target_directory
    }

    pub fn source_directory(&self) -> &PathBuf {
        &self.source_directory
    }
}