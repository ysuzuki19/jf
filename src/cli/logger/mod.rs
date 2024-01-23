mod log_level;
mod log_writer;

use crate::error::JfResult;

pub use self::log_level::LogLevel;
pub use log_writer::*;

pub struct Logger<LR: LogWriter> {
    level: LogLevel,
    log_writer: LR,
}

impl<LR: LogWriter> Clone for Logger<LR> {
    fn clone(&self) -> Self {
        Self {
            level: self.level,
            log_writer: LR::initialize(),
        }
    }
}

impl<LR: LogWriter> Default for Logger<LR> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<LR: LogWriter> PartialEq for Logger<LR> {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

impl<LR: LogWriter> Logger<LR> {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            log_writer: LR::initialize(),
        }
    }

    async fn write_with_guard<S: AsRef<str>>(&mut self, level: LogLevel, msg: S) -> JfResult<()> {
        if self.level >= level {
            self.log_writer.write(msg.as_ref()).await
        } else {
            Ok(())
        }
    }

    // Force to write log without log level guard
    pub async fn force<S: AsRef<str>>(&mut self, msg: S) -> JfResult<()> {
        self.log_writer.write(msg.as_ref()).await
    }

    // pub fn info<S: AsRef<str>>(&self, msg: S) {
    //     self.write_with_guard(LogLevel::Info, msg)
    // }

    // pub fn warn<S: AsRef<str>>(&self, msg: S) {
    //     self.write_with_guard(LogLevel::Warn, msg)
    // }

    pub async fn error<S: AsRef<str>>(&mut self, msg: S) -> JfResult<()> {
        self.write_with_guard(LogLevel::Error, msg).await
    }

    #[cfg(test)]
    #[cfg_attr(coverage, coverage(off))]
    pub fn level(&self) -> LogLevel {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::*;

    use self::log_writer::MockLogWriter;

    use super::*;

    impl Fixture for Logger<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Self::new(Default::default())
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn initialize() {
        let _ = Logger::<tokio::io::Stdout>::new(LogLevel::Info).clone();
        let _ = Logger::<MockLogWriter>::new(LogLevel::Info).clone();
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_under_info() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut logger = Logger::<MockLogWriter>::new(LogLevel::Info);
                assert!(logger.level() == LogLevel::Info);
                logger.force("log_msg").await?;
                logger.error("error_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 2);
                assert_eq!(logger.log_writer.lines[0], "log_msg");
                assert_eq!(logger.log_writer.lines[1], "error_msg");
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_under_error() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut logger = Logger::<MockLogWriter>::new(LogLevel::Error);
                assert!(logger.level() == LogLevel::Error);
                logger.force("log_msg").await?;
                logger.error("error_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 2);
                assert_eq!(logger.log_writer.lines[0], "log_msg");
                assert_eq!(logger.log_writer.lines[1], "error_msg");
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_under_none() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut logger = Logger::<MockLogWriter>::new(LogLevel::None);
                assert!(logger.level() == LogLevel::None);

                logger.force("log_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 1); // logger.log() is forced to display
                logger.error("").await?;
                assert_eq!(logger.log_writer.lines.len(), 1); // guard by log level
                Ok(())
            },
        )
    }
}
