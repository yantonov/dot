use crate::cli_arguments::{Arguments, BackupSubcommand};
use crate::cli_arguments::Command::{Backup, Link, List, Unlink, Check};
use std::process::exit;
use crate::environment::Environment;
use crate::log::{LogLevel};

mod environment;
mod cli_arguments;
mod handlers;
mod log;
mod util;

fn environment(args: &Arguments) -> Result<Environment, String> {
    let source_directory = args.source_directory()?;
    let target_directory = args.target_directory()?;
    environment::system_environment(
        &source_directory,
        &target_directory)
}

fn main() {
    let args = cli_arguments::arguments();
    let logger = log::create(args.verbose());
    match environment(&args) {
        Ok(env) => {
            let result = match args.command() {
                Link(_) => handlers::link(&env, &logger),
                Unlink(_) => handlers::unlink(&env, &logger),
                List(_) => handlers::list(&env, &logger),
                Backup(subcommand) => {
                    match subcommand.backup_subcommand() {
                        BackupSubcommand::List(_) =>
                            handlers::list_backup(&env, &logger),
                        BackupSubcommand::Remove(_) =>
                            handlers::remove_backup(&env, &logger),
                    }
                }
                Check(_) => handlers::check(&env, &logger),
            };
            exit(result.map_or(1, |_| 0));
        }
        Err(message) => {
            logger.log(LogLevel::Error, &message);
            exit(1)
        }
    }
}
