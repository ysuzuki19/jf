mod log_level;

pub use self::log_level::LogLevel;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq, Default))]
pub struct Logger {
    level: LogLevel,
    #[cfg(test)]
    pub(crate) log: std::cell::RefCell<Vec<String>>,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            #[cfg(test)]
            log: std::cell::RefCell::new(vec![]),
        }
    }

    fn write(&self, msg: &str) {
        #[cfg(not(test))]
        println!("{}", msg);
        #[cfg(test)]
        self.log.borrow_mut().push(msg.to_string());
    }

    fn display<S: AsRef<str>>(&self, level: LogLevel, msg: S) {
        if self.level >= level {
            self.write(msg.as_ref());
        }
    }

    // Force to display log without header regardless of log level
    pub fn log<S: AsRef<str>>(&self, msg: S) {
        self.write(msg.as_ref())
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

    #[cfg(test)]
    pub fn level(&self) -> LogLevel {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl crate::testutil::Fixture for Logger {
        fn gen() -> Self {
            Self::new(LogLevel::None)
        }
    }

    #[test]
    fn cover() {
        let _ = Logger::new(LogLevel::Info).clone();
    }

    #[test]
    fn test_under_info() {
        let logger = Logger::new(LogLevel::Info);
        assert_eq!(logger.level(), LogLevel::Info);
        logger.log("log_msg");
        logger.error("error_msg");
        assert_eq!(logger.log.borrow().len(), 2);
        assert_eq!(logger.log.borrow()[0], "log_msg");
        assert_eq!(logger.log.borrow()[1], "error_msg");
    }

    #[test]
    fn test_under_error() {
        let logger = Logger::new(LogLevel::Error);
        assert_eq!(logger.level(), LogLevel::Error);
        logger.log("log_msg");
        logger.error("error_msg");
        assert_eq!(logger.log.borrow().len(), 2);
        assert_eq!(logger.log.borrow()[0], "log_msg");
        assert_eq!(logger.log.borrow()[1], "error_msg");
    }

    #[test]
    fn test_under_none() {
        let logger = Logger::new(LogLevel::None);
        assert_eq!(logger.level(), LogLevel::None);
        logger.log("log_msg");
        logger.error("error_msg");
        assert_eq!(logger.log.borrow().len(), 1);
        assert_eq!(logger.log.borrow()[0], "log_msg");
    }
}
