use colored::Colorize;
use walkdir::DirEntry;

use crate::log::{LogLevel};
use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::util::to_result;

pub struct LoggedOperation<'a> {
    operation: &'a dyn FileOperation,
}

impl FileOperation for LoggedOperation<'_> {
    fn call(&self, context: &FileOperationContext<'_>, entry: &DirEntry) -> Result<(), String> {
        let result = self.operation.call(context, entry);
        let entry_path_str = to_result(entry.path().to_str(), "cannot get file name")?;
        if result.is_err() {
            context.logger().log(LogLevel::Error,
                                 &format!("{} - {}",
                                          entry_path_str,
                                          result.as_ref().unwrap_err()))
        } else {
            context.logger().log(LogLevel::Info,
                                 &format!("[{}] - {}", "Ok".green(), &entry_path_str.to_string()))
        }
        result
    }
}

impl LoggedOperation<'_> {
    pub fn wrap<'a>(
        operation: &'a dyn FileOperation,
    ) -> LoggedOperation<'a> {
        LoggedOperation {
            operation,
        }
    }
}