use walkdir::DirEntry;

use crate::log::{Logger, LogLevel};
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;

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
                                     result.as_ref().unwrap_err()))
        } else {
            self.logger.log(LogLevel::Info,
                            &format!("{}",
                                     entry_path_str))
        }
        result
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