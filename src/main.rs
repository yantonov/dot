use std::result::Result;

use crate::cli_arguments::Command::{Link, Unlink};

mod environment;
mod cli_arguments;
mod handlers;

fn main() -> Result<(), String> {
    let environment = environment::environment();
    let cli_arguments = cli_arguments::arguments();

    match cli_arguments.command() {
        Link(_) => handlers::link(&environment),
        Unlink(_) => handlers::unlink(&environment)
    }
}
