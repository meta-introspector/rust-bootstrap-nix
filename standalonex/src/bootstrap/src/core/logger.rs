use std::fmt;
use std::io::{self, Write};

pub struct Logger {
    verbose: u8,
}

impl Logger {
    pub fn new(verbose: u8) -> Self {
        Logger { verbose }
    }

    pub fn info(&self, message: &fmt::Arguments) {
        if self.verbose > 0 {
            self.log(message, "INFO", io::stdout());
        }
    }

    pub fn warn(&self, message: &fmt::Arguments) {
        if self.verbose > 0 {
            self.log(message, "WARN", io::stderr());
        }
    }

    pub fn error(&self, message: &fmt::Arguments) {
        self.log(message, "ERROR", io::stderr());
    }

    pub fn verbose(&self, level: u8, message: &fmt::Arguments) {
        if self.verbose >= level {
            self.log(message, "DEBUG", io::stdout());
        }
    }

    fn log(&self, message: &fmt::Arguments, level_str: &str, mut stream: impl Write) {
        let _ = writeln!(stream, "[{}] {}", level_str, message);
    }
}
