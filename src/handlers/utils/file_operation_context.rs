use std::path::Path;

use crate::environment::Environment;
use crate::log::Logger;

pub struct FileOperationContext<'a> {
    target_directory: &'a Path,
    source_directory: &'a Path,
    logger: &'a Logger,
    dry_run: bool,
}

impl<'a> FileOperationContext<'a> {
    pub fn create(env: &'a Environment, logger: &'a Logger, dry_run: bool) -> FileOperationContext<'a> {
        FileOperationContext {
            target_directory: env.target_directory(),
            source_directory: env.source_directory(),
            logger,
            dry_run,
        }
    }

    pub fn target_directory(&self) -> &Path {
        self.target_directory
    }

    pub fn source_directory(&self) -> &Path {
        self.source_directory
    }

    pub fn logger(&self) -> &Logger {
        self.logger
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}