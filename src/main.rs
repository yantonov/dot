use crate::cli_arguments::Command::{Link, List, ListBackup, RemoveBackup, Unlink};

mod environment;
mod cli_arguments;
mod handlers;
mod log;

fn main() {
    let environment = environment::system_environment();
    let cli_arguments = cli_arguments::arguments();
    let logger = log::create(cli_arguments.verbose());

    match cli_arguments.command() {
        Link(_) => handlers::link(&environment, &logger),
        Unlink(_) => handlers::unlink(&environment, &logger),
        List(_) => handlers::list(&environment, &logger),
        ListBackup(_) => handlers::list_backup(&environment, &logger),
        RemoveBackup(_) => handlers::remove_backup(&environment, &logger),
    }
}
