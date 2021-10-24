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
use crate::handlers::operations::check_operation::CheckFileOperation;

mod operations;
mod utils;

fn file_iteration_handler(environment: &Environment,
                          operation: &dyn FileOperation<Context=FileOperationContext>) -> Result<(), String> {
    iterate_files(environment.source_directory(),
                  &FileOperationContext::create(environment),
                  operation)
}

pub fn check(environment: &Environment,
             logger: &Logger) -> Result<(), String> {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &CheckFileOperation {}))
}

pub fn link(environment: &Environment,
            logger: &Logger) -> Result<(), String> {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &LinkFileOperation {}))
}

pub fn unlink(environment: &Environment,
              logger: &Logger) -> Result<(), String> {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &UnlinkFileOperation {}))
}

pub fn list(environment: &Environment,
            logger: &Logger) -> Result<(), String> {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &ListFileOperation {}))
}

pub fn list_backup(environment: &Environment,
                   logger: &Logger) -> Result<(), String> {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &ListBackupOperation {}))
}

pub fn remove_backup(environment: &Environment,
                     logger: &Logger) -> Result<(), String> {
    file_iteration_handler(environment,
                           &LoggedOperation::wrap(logger,
                                                  &RemoveBackupOperation {}))
}