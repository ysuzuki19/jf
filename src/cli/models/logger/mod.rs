mod log_level;

pub use self::log_level::LogLevel;

#[derive(Clone)]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    fn display<S: AsRef<str>>(&self, level: LogLevel, msg: S) {
        if self.level >= level {
            println!("{}", msg.as_ref())
        }
    }

    pub fn log<S: AsRef<str>>(&self, msg: S) {
        println!("{}", msg.as_ref())
    }

    // pub fn info<S: AsRef<str>>(&self, msg: S) {
    //     self.display(LogLevel::Info, msg)
    // }

    // pub fn warn<S: AsRef<str>>(&self, msg: S) {
    //     self.display(LogLevel::Warn, msg)
    // }

    pub fn error<S: AsRef<str>>(&self, msg: S) {
        self.display(LogLevel::Error, msg)
    }
}
