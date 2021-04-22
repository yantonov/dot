use operations::link_operation::LinkFileOperation;
use operations::list_backup_operation::ListBackupOperation;
use operations::list_operation::ListFileOperation;
use operations::remove_backup_operation::RemoveBackupOperation;
use operations::unlink_operation::UnlinkFileOperation;
use utils::logged_operation::LoggedOperation;

use crate::environment::Environment;
use crate::log::Logger;
use crate::handlers::utils::file_operation::{iterate_files, FileOperation};
use crate::handlers::utils::file_operation_context::FileOperationContext;

mod operations;
mod utils;

fn file_iteration_handler(environment: &Environment,
                          operation: &dyn FileOperation<Context=FileOperationContext>) {
    iterate_files(environment.current_dir(),
                  &FileOperationContext::create(environment),
                  operation)
}

pub fn link(environment: &Environment,
            logger: &Logger) {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &LinkFileOperation {}))
}

pub fn unlink(environment: &Environment,
              logger: &Logger) {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &UnlinkFileOperation {}))
}

pub fn list(environment: &Environment,
            _: &Logger) {
    file_iteration_handler(environment,
                           &ListFileOperation {})
}

pub fn list_backup(environment: &Environment,
                   _: &Logger) {
    file_iteration_handler(environment,
                           &ListBackupOperation {})
}

pub fn remove_backup(environment: &Environment,
                     _: &Logger) {
    file_iteration_handler(environment,
                           &RemoveBackupOperation {})
}
