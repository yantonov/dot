use std::path::PathBuf;

use crate::environment::Environment;

pub struct FileOperationContext {
    home: PathBuf,
    current_directory: PathBuf,
}

impl FileOperationContext {
    pub fn create(env: &Environment) -> FileOperationContext {
        let home = env.home_directory().clone();
        let current_directory = env.current_dir().clone();

        FileOperationContext {
            home,
            current_directory,
        }
    }

    pub fn home(&self) -> &PathBuf {
        &self.home
    }

    pub fn current_directory(&self) -> &PathBuf {
        &self.current_directory
    }
}