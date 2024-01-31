mod log_level;
mod log_writer;

use crate::util::error::JfResult;

pub use self::log_level::LogLevel;
pub use log_writer::*;

#[cfg_attr(test, derive(Debug))]
pub struct Logger<LR: LogWriter> {
    level: LogLevel,
    log_writer: LR,
}

impl<LR: LogWriter> Clone for Logger<LR> {
    fn clone(&self) -> Self {
        Self {
            level: self.level,
            log_writer: LR::init(),
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
            log_writer: LR::init(),
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

    pub async fn debug<S: AsRef<str>>(&mut self, msg: S) -> JfResult<()> {
        self.write_with_guard(LogLevel::Debug, msg).await
    }

    pub async fn info<S: AsRef<str>>(&mut self, msg: S) -> JfResult<()> {
        self.write_with_guard(LogLevel::Info, msg).await
    }

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
    use crate::util::testutil::*;

    use super::*;

    impl Fixture for Logger<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Self::new(Fixture::fixture())
        }
    }

    #[cfg(test)]
    impl<LR: LogWriter> Logger<LR> {
        #[cfg_attr(coverage, coverage(off))]
        pub fn log_writer(&self) -> &LR {
            &self.log_writer
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn init() {
        let _ = Logger::<JfStdout>::new(LogLevel::Info).clone();
        let _ = Logger::<MockLogWriter>::new(LogLevel::Info).clone();
        assert_eq!(Logger::<JfStdout>::default().level(), LogLevel::default());
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_under_info() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut logger = Logger::<MockLogWriter>::new(LogLevel::Info);
                assert_eq!(logger.level(), LogLevel::Info);
                logger.force("log_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 1);
                assert_eq!(logger.log_writer.lines[0], "log_msg");
                logger.error("error_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 2);
                assert_eq!(logger.log_writer.lines[1], "error_msg");
                logger.error("info_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 3);
                assert_eq!(logger.log_writer.lines[2], "info_msg");
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
                assert_eq!(logger.level(), LogLevel::Error);
                logger.force("log_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 1);
                assert_eq!(logger.log_writer.lines[0], "log_msg");
                logger.error("error_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 2);
                assert_eq!(logger.log_writer.lines[1], "error_msg");
                logger.info("info_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 2);
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
                assert_eq!(logger.level(), LogLevel::None);

                logger.force("log_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 1); // logger.log() is forced to display
                logger.info("info_msg").await?;
                assert_eq!(logger.log_writer.lines.len(), 1); // guard by log level
                logger.error("").await?;
                assert_eq!(logger.log_writer.lines.len(), 1); // guard by log level
                Ok(())
            },
        )
    }
}
