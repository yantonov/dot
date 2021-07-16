use crate::cli_arguments::{BackupSubcommand};
use crate::cli_arguments::Command::{Backup, Link, List, Unlink, Check};
use std::process::exit;

mod environment;
mod cli_arguments;
mod handlers;
mod log;
mod util;

fn main() {
    let environment = environment::system_environment();
    let cli_arguments = cli_arguments::arguments();
    let logger = log::create(cli_arguments.verbose());

    let result = match cli_arguments.command() {
        Link(_) => handlers::link(&environment, &logger),
        Unlink(_) => handlers::unlink(&environment, &logger),
        List(_) => handlers::list(&environment, &logger),
        Backup(subcommand) => {
            match subcommand.backup_subcommand() {
                BackupSubcommand::List(_) =>
                    handlers::list_backup(&environment, &logger),
                BackupSubcommand::Remove(_) =>
                    handlers::remove_backup(&environment, &logger),
            }
        }
        Check(_) => handlers::check(&environment, &logger),
    };
    exit(result.map_or(1, |_| 0));
}
