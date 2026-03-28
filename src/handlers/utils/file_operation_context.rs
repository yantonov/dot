use std::path::{Path, PathBuf};

use crate::environment::Environment;
use crate::log::Logger;

pub struct FileOperationContext<'a> {
    target_directory: PathBuf,
    source_directory: PathBuf,
    logger: &'a Logger,
}



impl<'a> FileOperationContext<'a> {
    pub fn create(env: &'a Environment, logger: &'a Logger) -> FileOperationContext<'a> {
        let target_directory = env.target_directory().to_path_buf();
        let source_directory = env.source_directory().to_path_buf();

        FileOperationContext {
            target_directory,
            source_directory,
            logger,
        }
    }

    pub fn target_directory(&self) -> &Path {
        &self.target_directory
    }

    pub fn source_directory(&self) -> &Path {
        &self.source_directory
    }

    pub fn logger(&self) -> &Logger {
        self.logger
    }
}