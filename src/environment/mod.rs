use std::env;
use std::path::PathBuf;

pub struct Environment {}

impl Environment {
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

pub fn environment() -> Environment {
    return Environment {};
}
