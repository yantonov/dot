use operations::link_operation::LinkFileOperation;
use operations::list_backup_operation::ListBackupOperation;
use operations::list_operation::ListFileOperation;
use operations::remove_backup_operation::RemoveBackupOperation;
use operations::unlink_operation::UnlinkFileOperation;
use utils::logged_operation::LoggedOperation;

use crate::environment::Environment;
use crate::handlers::operations::check_operation::CheckFileOperation;
use crate::handlers::utils::file_operation::{FileOperation, iterate_files};
use crate::handlers::utils::file_operation_context::FileOperationContext;
use crate::handlers::utils::symlink_support::check_symlink_support;
use crate::log::{LogLevel, Logger};

mod operations;
mod utils;

fn file_iteration_handler(
    environment: &Environment,
    logger: &Logger,
    dry_run: bool,
    operation: &dyn FileOperation,
) -> Result<(), String> {
    iterate_files(
        environment.source_directory(),
        &FileOperationContext::create(environment, logger, dry_run),
        operation,
    )
}

pub fn check(environment: &Environment, logger: &Logger) -> Result<(), String> {
    file_iteration_handler(
        environment,
        logger,
        false,
        &LoggedOperation::wrap(&CheckFileOperation {}),
    )
}

pub fn link(environment: &Environment, logger: &Logger, dry_run: bool) -> Result<(), String> {
    // only per file errors reach the log on their own, this one has to report itself
    if !dry_run && let Err(message) = check_symlink_support(environment.target_directory()) {
        logger.log(LogLevel::Error, &message);
        return Err(message);
    }
    file_iteration_handler(
        environment,
        logger,
        dry_run,
        &LoggedOperation::wrap(&LinkFileOperation {}),
    )
}

pub fn unlink(environment: &Environment, logger: &Logger, dry_run: bool) -> Result<(), String> {
    file_iteration_handler(
        environment,
        logger,
        dry_run,
        &LoggedOperation::wrap(&UnlinkFileOperation {}),
    )
}

pub fn list(environment: &Environment, logger: &Logger) -> Result<(), String> {
    file_iteration_handler(
        environment,
        logger,
        false,
        &LoggedOperation::wrap(&ListFileOperation {}),
    )
}

pub fn list_backup(environment: &Environment, logger: &Logger) -> Result<(), String> {
    file_iteration_handler(
        environment,
        logger,
        false,
        &LoggedOperation::wrap(&ListBackupOperation {}),
    )
}

pub fn remove_backup(
    environment: &Environment,
    logger: &Logger,
    dry_run: bool,
) -> Result<(), String> {
    file_iteration_handler(
        environment,
        logger,
        dry_run,
        &LoggedOperation::wrap(&RemoveBackupOperation {}),
    )
}
