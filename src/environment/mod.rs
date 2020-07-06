use std::env;
use std::path::PathBuf;

pub struct Environment {
    current_dir: PathBuf,
    home_directory: String,
}

impl Environment {
    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn home_directory(&self) -> &String {
        &self.home_directory
    }
}

struct SystemEnvironment {}

impl SystemEnvironment {
    pub fn current_dir(&self) -> Result<PathBuf, String> {
        let current_dir = env::current_dir()
            .map_err(|_| "cannot get current directory")?;
        Ok(current_dir)
    }

    pub fn home_directory(&self) -> Result<String, &str> {
        return env::var("HOME")
            .map_err(|_| "HOME environment variable is not defined");
    }
}

pub fn system_environment() -> Environment {
    let sys_env = SystemEnvironment {};
    return Environment {
        current_dir: sys_env.current_dir().unwrap(),
        home_directory: sys_env.home_directory().unwrap(),
    };
}
