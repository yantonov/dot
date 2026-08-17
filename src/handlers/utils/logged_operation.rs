use walkdir::DirEntry;

use crate::handlers::utils::file_operation::FileOperation;
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::log::{LogLevel, green};

pub struct LoggedOperation<'a> {
    operation: &'a dyn FileOperation,
}

impl FileOperation for LoggedOperation<'_> {
    fn call(&self, context: &FileOperationContext<'_>, entry: &DirEntry) -> Result<(), String> {
        let result = self.operation.call(context, entry);
        let entry_path_str = entry.path().to_str().ok_or("cannot get file name")?;
        if let Err(e) = &result {
            context
                .logger()
                .log(LogLevel::Error, &format!("{} - {}", entry_path_str, e))
        } else {
            context.logger().log(
                LogLevel::Info,
                &format!("[{}] - {}", green("Ok"), entry_path_str),
            )
        }
        result
    }
}

impl LoggedOperation<'_> {
    pub fn wrap<'a>(operation: &'a dyn FileOperation) -> LoggedOperation<'a> {
        LoggedOperation { operation }
    }
}
