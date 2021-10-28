use std::env;
use std::path::{Path, PathBuf};

pub struct Environment {
    source_directory: PathBuf,
    target_directory: PathBuf,
}

impl Environment {
    pub fn source_directory(&self) -> &PathBuf {
        &self.source_directory
    }

    pub fn target_directory(&self) -> &PathBuf {
        &self.target_directory
    }
}

struct DefaultEnvironment {}

impl DefaultEnvironment {
    fn source_directory(&self) -> Result<PathBuf, String> {
        env::current_dir()
            .map_err(|_| "cannot get current directory".to_string())
    }

    fn target_directory(&self) -> Result<PathBuf, String> {
        env::var("HOME")
            .map(|home| Path::new(&home).to_path_buf())
            .map_err(|_| "HOME environment variable is not defined".to_string())
    }
}

pub fn system_environment(source: &Option<PathBuf>,
                          target: &Option<PathBuf>) -> Result<Environment, String> {
    let default_env = DefaultEnvironment {};
    Ok(Environment {
        source_directory: source.clone()
            .unwrap_or(default_env.source_directory()?),
        target_directory: target.clone()
            .unwrap_or(default_env.target_directory()?),
    })
}
