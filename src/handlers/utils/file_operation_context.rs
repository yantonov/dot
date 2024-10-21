use std::path::PathBuf;

use crate::environment::Environment;
use crate::log::Logger;

pub struct FileOperationContext<'a> {
    target_directory: PathBuf,
    source_directory: PathBuf,
    logger: &'a Logger,
}



impl<'a> FileOperationContext<'a> {
    pub fn create(env: &'a Environment, logger: &'a Logger) -> FileOperationContext<'a> {
        let target_directory = env.target_directory().clone();
        let source_directory = env.source_directory().clone();

        FileOperationContext {
            target_directory,
            source_directory,
            logger,
        }
    }

    pub fn target_directory(&self) -> &PathBuf {
        &self.target_directory
    }

    pub fn source_directory(&self) -> &PathBuf {
        &self.source_directory
    }

    pub fn logger(&self) -> &Logger {
        self.logger
    }
}