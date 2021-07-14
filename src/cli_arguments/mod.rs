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
    #[clap(about = "create symbolic links, backup files will be generated", display_order = 0)]
    Link(Link),
    #[clap(about = "remove symbolic links, and use regular files", display_order = 1)]
    Unlink(Unlink),
    #[clap(about = "list files (recursively) inside the current directory", display_order = 2)]
    List(List),
    #[clap(about = "backup commands", display_order = 3)]
    Backup(Backup),
    #[clap(about = "check that all links exists", display_order = 4)]
    Check(Check)
}

#[derive(Clap)]
pub struct Link {}

#[derive(Clap)]
pub struct Unlink {}

#[derive(Clap)]
pub struct List {}

#[derive(Clap)]
pub struct Backup {
    #[clap(subcommand)]
    subcommand: BackupSubcommand
}

impl Backup {
    pub fn backup_subcommand(&self) -> &BackupSubcommand {
        return &self.subcommand;
    }
}

#[derive(Clap)]
pub enum BackupSubcommand {
    #[clap(about = "list backup files", display_order = 0)]
    List(ListBackup),
    #[clap(about = "remove backup files", display_order = 1)]
    Remove(RemoveBackup),
}

#[derive(Clap)]
pub struct ListBackup {}

#[derive(Clap)]
pub struct RemoveBackup {}

#[derive(Clap)]
pub struct Check {}

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
