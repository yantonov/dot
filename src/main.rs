use std::result::Result;

use crate::cli_arguments::Command::{Link, Unlink, List};

mod environment;
mod cli_arguments;
mod handlers;
mod log;

fn main() -> Result<(), String> {
    let environment = environment::system_environment();
    let cli_arguments = cli_arguments::arguments();
    let logger = log::create(cli_arguments.verbose());

    match cli_arguments.command() {
        Link(_) => handlers::link(&environment, &logger),
        Unlink(_) => handlers::unlink(&environment, &logger),
        List(_) => handlers::list(&environment, &logger)
    }
}
