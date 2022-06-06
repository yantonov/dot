use std::fs;
use std::path::{PathBuf};
use clap::Parser;

#[derive(Parser)]
#[clap(version)]
struct Opts {
    #[clap(short, long, parse(from_occurrences), help="Verbose output")]
    verbose: i32,
    #[clap(subcommand)]
    command: Command,
    #[clap(short, long, help="Repository/source path")]
    source: Option<String>,
    #[clap(short, long, help="Target path")]
    target: Option<String>,
}

#[derive(Parser)]
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
    Check(Check),
}

#[derive(Parser)]
pub struct Link {}

#[derive(Parser)]
pub struct Unlink {}

#[derive(Parser)]
pub struct List {}

#[derive(Parser)]
pub struct Backup {
    #[clap(subcommand)]
    subcommand: BackupSubcommand,
}

impl Backup {
    pub fn backup_subcommand(&self) -> &BackupSubcommand {
        &self.subcommand
    }
}

#[derive(Parser)]
pub enum BackupSubcommand {
    #[clap(about = "list backup files", display_order = 0)]
    List(ListBackup),
    #[clap(about = "remove backup files", display_order = 1)]
    Remove(RemoveBackup),
}

#[derive(Parser)]
pub struct ListBackup {}

#[derive(Parser)]
pub struct RemoveBackup {}

#[derive(Parser)]
pub struct Check {}

pub struct Arguments {
    args: Opts,
}

fn validate_dir(path: &Option<String>, title: &str) -> Result<Option<PathBuf>, String> {
    match path {
        None => Ok(None),
        Some(p) => {
            let path_buf = PathBuf::from(p.clone());
            if !path_buf.exists() {
                Err(format!("Path '{}' '{}' does not exists", title, p))
            }
            else {
                let canonical_path_buf = fs::canonicalize(&path_buf)
                    .map_err(|_| format!("cannot canonicalize '{}', '{}'", title, p))?;
                if !canonical_path_buf.exists() {
                    Err(format!("'{}' '{}' does not exists", title, p))
                } else {
                    Ok(Some(canonical_path_buf))
                }
            }
        }
    }
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }

    pub fn verbose(&self) -> i32 { self.args.verbose }

    pub fn target_directory(&self) -> Result<Option<PathBuf>, String> {
        validate_dir(&self.args.target, "target directory")
    }

    pub fn source_directory(&self) -> Result<Option<PathBuf>, String> {
        validate_dir(&self.args.source, "source directory")
    }
}

pub fn arguments() -> Arguments {
    Arguments { args: Opts::parse() }
}
