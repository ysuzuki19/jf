mod log_level;
mod log_writer;

pub use log_level::LogLevel;

use tokio::sync::mpsc;

use crate::util::error::JfResult;

pub use self::log_writer::JfStdout;
use self::log_writer::LogWriter;
#[cfg(test)]
pub use tests::LogWorkerMock;

pub struct LogMessage {
    pub line: String,
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Logger {
    tx: mpsc::Sender<LogMessage>,
    log_level: LogLevel,
}

#[cfg(test)]
impl PartialEq for Logger {
    fn eq(&self, other: &Self) -> bool {
        self.log_level == other.log_level
    }
}

impl Logger {
    #[cfg(test)]
    pub fn level(&self) -> LogLevel {
        self.log_level
    }

    #[cfg(test)]
    pub fn update(&mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
        self.clone()
    }

    async fn send(&mut self, msg: LogMessage) -> JfResult<()> {
        self.tx.send(msg).await?;
        Ok(())
    }

    async fn send_with_guard(&mut self, log_level: LogLevel, msg: LogMessage) -> JfResult<()> {
        if self.log_level >= log_level {
            self.send(msg).await?;
        }
        Ok(())
    }

    pub async fn force<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send(LogMessage {
            line: line.as_ref().to_string(),
        })
        .await?;
        Ok(())
    }

    pub async fn error<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Error,
            LogMessage {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn warn<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Warn,
            LogMessage {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }

    pub async fn info<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Info,
            LogMessage {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }

    pub async fn debug<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Debug,
            LogMessage {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }
}

pub struct LogWorker {
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl LogWorker {
    pub fn new() -> Self {
        Self { handle: None }
    }

    pub async fn start<LR: LogWriter>(
        &mut self,
        mut log_writer: LR,
        log_level: LogLevel,
    ) -> Logger {
        let (tx, mut rx) = mpsc::channel::<LogMessage>(100);
        self.handle = Some(tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                log_writer.write(&msg.line).await.unwrap();
            }
        }));
        Logger { tx, log_level }
    }

    pub async fn join(&mut self) -> JfResult<()> {
        self.handle.take().unwrap().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tests::log_writer::MockLogWriter;

    use crate::util::testutil::{async_test, Fixture};

    use super::*;

    pub struct LogWorkerMock {
        pub worker: LogWorker,
        pub logger: Logger,
        pub log_writer: MockLogWriter,
    }
    impl LogWorkerMock {
        pub async fn new() -> Self {
            let log_writer = MockLogWriter::new();
            let mut log_worker = LogWorker::new();
            let logger = log_worker
                .start(log_writer.clone(), Fixture::fixture())
                .await;
            Self {
                worker: log_worker,
                logger,
                log_writer,
            }
        }
    }

    #[test]
    fn test_log_debug() -> JfResult<()> {
        async_test(async move {
            let log_writer = MockLogWriter::new();
            let mut log_worker = LogWorker::new();
            {
                let mut logger = log_worker
                    .start(log_writer.clone(), Fixture::fixture())
                    .await
                    .update(LogLevel::Debug);
                logger.error("error".to_string()).await?;
                logger.warn("warn".to_string()).await?;
                logger.info("info".to_string()).await?;
                logger.debug("debug".to_string()).await?;
            } // force to drop logger
            log_worker.join().await?;
            assert_eq!(log_writer.lines(), vec!["error", "warn", "info", "debug"]);
            Ok(())
        })
    }

    #[test]
    fn test_log_info() -> JfResult<()> {
        async_test(async move {
            let log_writer = MockLogWriter::new();
            let mut log_worker = LogWorker::new();
            {
                let mut logger = log_worker
                    .start(log_writer.clone(), Fixture::fixture())
                    .await
                    .update(LogLevel::Info);
                logger.error("error".to_string()).await?;
                logger.warn("warn".to_string()).await?;
                logger.info("info".to_string()).await?;
                logger.debug("debug".to_string()).await?;
            }
            log_worker.join().await?;
            assert_eq!(log_writer.lines(), vec!["error", "warn", "info"]);
            Ok(())
        })
    }

    #[test]
    fn test_log_warn() -> JfResult<()> {
        async_test(async move {
            let log_writer = MockLogWriter::new();
            let mut log_worker = LogWorker::new();
            {
                let mut logger = log_worker
                    .start(log_writer.clone(), Fixture::fixture())
                    .await
                    .update(LogLevel::Warn);
                logger.error("error".to_string()).await?;
                logger.warn("warn".to_string()).await?;
                logger.info("info".to_string()).await?;
                logger.debug("debug".to_string()).await?;
            }
            log_worker.join().await?;
            assert_eq!(log_writer.lines(), vec!["error", "warn"]);
            Ok(())
        })
    }

    #[test]
    fn test_log_error() -> JfResult<()> {
        async_test(async move {
            let log_writer = MockLogWriter::new();
            let mut log_worker = LogWorker::new();
            {
                let mut logger = log_worker
                    .start(log_writer.clone(), Fixture::fixture())
                    .await
                    .update(LogLevel::Error);
                logger.error("error".to_string()).await?;
                logger.warn("warn".to_string()).await?;
                logger.info("info".to_string()).await?;
                logger.debug("debug".to_string()).await?;
            }
            log_worker.join().await?;
            assert_eq!(log_writer.lines(), vec!["error"]);
            Ok(())
        })
    }

    #[test]
    fn test_log_none() -> JfResult<()> {
        async_test(async move {
            let log_writer = MockLogWriter::new();
            let mut log_worker = LogWorker::new();
            {
                let mut logger = log_worker
                    .start(log_writer.clone(), Fixture::fixture())
                    .await
                    .update(LogLevel::None);
                logger.error("error".to_string()).await?;
                logger.warn("warn".to_string()).await?;
                logger.info("info".to_string()).await?;
                logger.debug("debug".to_string()).await?;
            }
            log_worker.join().await?;
            assert_eq!(log_writer.lines(), Vec::<String>::new());
            Ok(())
        })
    }
}
