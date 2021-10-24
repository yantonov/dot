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

struct SystemEnvironment {}

impl SystemEnvironment {
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

pub fn system_environment() -> Environment {
    let sys_env = SystemEnvironment {};
    return Environment {
        source_directory: sys_env.source_directory().unwrap(),
        target_directory: sys_env.target_directory().unwrap(),
    };
}
