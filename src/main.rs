use crate::cli_arguments::Command::{Backup, Check, Link, List, Unlink};
use crate::cli_arguments::BackupSubcommand;
use crate::log::LogLevel;
use std::process::exit;

mod environment;
mod cli_arguments;
mod handlers;
mod log;

fn main() {
    let args = cli_arguments::arguments();
    let dry_run = args.dry_run();
    // dry-run implies verbose: the point of a dry run is to see what would happen
    let logger = log::create(args.verbose() || dry_run);
    match cli_arguments::environment(&args) {
        Ok(env) => {
            let result = match args.command() {
                Link(_) => handlers::link(&env, &logger, dry_run),
                Unlink(_) => handlers::unlink(&env, &logger, dry_run),
                List(_) => handlers::list(&env, &logger),
                Backup(subcommand) => {
                    match subcommand.backup_subcommand() {
                        BackupSubcommand::List(_) =>
                            handlers::list_backup(&env, &logger),
                        BackupSubcommand::Remove(_) =>
                            handlers::remove_backup(&env, &logger, dry_run),
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
