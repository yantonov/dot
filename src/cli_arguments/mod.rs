use clap::{Clap, crate_version};

#[derive(Clap)]
#[clap(version = crate_version ! ())]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
pub enum Command {
    #[clap(about = "create symbolic links, backup files will be generated")]
    Link(Link),
    #[clap(about = "remove symbolic links, and use regular files")]
    Unlink(Unlink),
    #[clap(about = "list files (recursively) inside the current directory")]
    List(List),
}

#[derive(Clap)]
pub struct Link {}

#[derive(Clap)]
pub struct Unlink {}

#[derive(Clap)]
pub struct List {}

pub struct Arguments {
    args: Opts
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }

    pub fn verbose(&self) -> i32 { self.args.verbose }
}

pub fn arguments() -> Arguments {
    return Arguments { args: Opts::parse() };
}
