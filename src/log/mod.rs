pub struct Logger {
    verbose: i32
}

impl Logger {
    pub fn log(&self, message: &str) {
        if self.verbose > 0 {
            println!("{}", message);
        }
    }
}

pub fn create(verbose: i32) -> Logger {
    Logger { verbose }
}