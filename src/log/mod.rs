use std::io::IsTerminal;

pub struct Logger {
    verbose: bool,
}

pub enum LogLevel {
    Info,
    Error,
}

impl Logger {
    pub fn log_dry_run_plan(&self, message: &str) {
        self.log(LogLevel::Info, &format!("[DRY RUN] {}", message));
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        match level {
            LogLevel::Info => {
                if self.verbose {
                    self.log_internal(level, message);
                }
            }
            LogLevel::Error => self.log_internal(level, message),
        }
    }

    fn log_internal(&self, level: LogLevel, message: &str) {
        println!("{} {}", self.level_token(level), message);
    }

    fn level_token(&self, level: LogLevel) -> String {
        match level {
            LogLevel::Info => "".to_string(),
            LogLevel::Error => red("[Error]"),
        }
    }
}

pub fn create(verbose: bool) -> Logger {
    Logger { verbose }
}

// A hand-rolled ANSI wrapper instead of a color crate: only two tokens ever
// need coloring, and clap already links its own terminal-color handling for
// its help/error output, so a second full color crate would be redundant.
fn colorize(code: &str, text: &str) -> String {
    if std::io::stdout().is_terminal() {
        format!("\x1b[{}m{}\x1b[0m", code, text)
    } else {
        text.to_string()
    }
}

pub fn red(text: &str) -> String {
    colorize("31", text)
}

pub fn green(text: &str) -> String {
    colorize("32", text)
}
