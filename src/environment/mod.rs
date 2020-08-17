use std::env;
use std::path::{Path, PathBuf};

pub struct Environment {
    current_dir: PathBuf,
    home_directory: PathBuf,
}

impl Environment {
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn home_directory(&self) -> &PathBuf {
        &self.home_directory
    }
}

struct SystemEnvironment {}

impl SystemEnvironment {
    fn current_dir(&self) -> Result<PathBuf, String> {
        env::current_dir()
            .map_err(|_| "cannot get current directory".to_string())
    }

    fn home_directory(&self) -> Result<PathBuf, String> {
        env::var("HOME")
            .map(|home| Path::new(&home).to_path_buf())
            .map_err(|_| "HOME environment variable is not defined".to_string())
    }
}

pub fn system_environment() -> Environment {
    let sys_env = SystemEnvironment {};
    return Environment {
        current_dir: sys_env.current_dir().unwrap(),
        home_directory: sys_env.home_directory().unwrap(),
    };
}
