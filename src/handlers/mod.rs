use link_operation::LinkFileOperation;
use list_backup_operation::ListBackupOperation;
use list_operation::ListFileOperation;
use logged_operation::LoggedOperation;
use remove_backup_operation::RemoveBackupOperation;
use unlink_operation::UnlinkFileOperation;

use crate::environment::Environment;
use crate::handlers::file_operation::{FileOperation, iterate_files};
use crate::handlers::file_operation_context::FileOperationContext;
use crate::log::Logger;

mod file_operation;
mod file_operation_context;
mod logged_operation;
mod link_operation;
mod unlink_operation;
mod list_operation;
mod list_backup_operation;
mod remove_backup_operation;
mod backup;
mod file_utils;

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
