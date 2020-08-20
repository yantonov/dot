use walkdir::DirEntry;

use crate::handlers::file_operation::FileOperation;
use crate::handlers::file_operation_context::FileOperationContext;
use crate::log::{Logger, LogLevel};

pub struct LoggedOperation<'a, TContext> {
    logger: &'a Logger,
    operation: &'a dyn FileOperation<Context=TContext>,
}

impl FileOperation for LoggedOperation<'_, FileOperationContext> {
    type Context = FileOperationContext;

    fn call(&self, context: &Self::Context, entry: &DirEntry) -> Result<(), String> {
        let result = self.operation.call(context, &entry);
        let entry_path_str = entry.path().to_str().unwrap();
        if result.is_err() {
            self.logger.log(LogLevel::Error,
                            &format!("{} - {}",
                                     entry_path_str,
                                     result.unwrap_err()))
        } else {
            self.logger.log(LogLevel::Info,
                            &format!("{}",
                                     entry_path_str))
        }
        Ok(())
    }
}

impl LoggedOperation<'_, FileOperationContext> {
    pub fn wrap<'a>(
        logger: &'a Logger,
        operation: &'a dyn FileOperation<Context=FileOperationContext>,
    ) -> LoggedOperation<'a, FileOperationContext> {
        LoggedOperation {
            logger,
            operation,
        }
    }
}