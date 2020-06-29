use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Yury Antonov <yantonov@yandex.ru>")]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    #[allow(dead_code)]
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
}

#[derive(Clap)]
pub struct Link {}

#[derive(Clap)]
pub struct Unlink {}


pub struct Arguments {
    args: Opts
}

impl Arguments {
    pub fn command(&self) -> &Command {
        return &self.args.command;
    }
}

pub fn arguments() -> Arguments {
    return Arguments { args: Opts::parse() };
}
