use colored::{ColoredString, Colorize};

pub struct Logger {
    verbose: i32
}

pub enum LogLevel {
    Info,
    Error,
}

impl Logger {
    pub fn log(&self, level: LogLevel, message: &str) {
        match level {
            LogLevel::Info => {
                if self.verbose > 0 {
                    self.log_internal(level, message);
                }
            }
            LogLevel::Error => {
                self.log_internal(level, message)
            }
        }
    }

    fn log_internal(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", self.level_token(level), message);
    }

    fn level_token(&self, level: LogLevel) -> ColoredString {
        match level {
            LogLevel::Info => "Ok".green(),
            LogLevel::Error => "Error".red(),
        }
    }
}

pub fn create(verbose: i32) -> Logger {
    Logger { verbose }
}