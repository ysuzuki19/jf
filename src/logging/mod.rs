// SPDX-License-Identifier: MPL-2.0
mod logger;
mod worker;

pub use logger::{log_generator, LogLevel, Logger};
pub use worker::{Stdout, Worker};

#[cfg(test)]
pub use tests::LoggingMock;

#[cfg(test)]
mod tests {
    use crate::util::{error::JfResult, testutil::*};

    use self::worker::Mock;

    use super::*;

    pub struct LoggingMock {
        pub _worker: Worker,
        pub logger: Logger,
        pub log_writer: Mock,
    }
    impl LoggingMock {
        pub async fn new() -> Self {
            let log_writer = Mock::new();
            let mut log_worker = Worker::new();
            let logger = log_worker
                .start(log_writer.clone(), Fixture::fixture())
                .await;
            Self {
                _worker: log_worker,
                logger,
                log_writer,
            }
        }
    }

    #[test]
    fn log_debug() -> JfResult<()> {
        async_test(async move {
            let log_writer = Mock::new();
            let mut log_worker = Worker::new();
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
            assert_eq!(
                log_writer.lines(),
                vec!["[E] error", "[W] warn", "[I] info", "[D] debug"]
            );
            Ok(())
        })
    }

    #[test]
    fn log_info() -> JfResult<()> {
        async_test(async move {
            let log_writer = Mock::new();
            let mut log_worker = Worker::new();
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
            assert_eq!(
                log_writer.lines(),
                vec!["[E] error", "[W] warn", "[I] info"]
            );
            Ok(())
        })
    }

    #[test]
    fn log_warn() -> JfResult<()> {
        async_test(async move {
            let log_writer = Mock::new();
            let mut log_worker = Worker::new();
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
            assert_eq!(log_writer.lines(), vec!["[E] error", "[W] warn"]);
            Ok(())
        })
    }

    #[test]
    fn log_error() -> JfResult<()> {
        async_test(async move {
            let log_writer = Mock::new();
            let mut log_worker = Worker::new();
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
            assert_eq!(log_writer.lines(), vec!["[E] error"]);
            Ok(())
        })
    }

    #[test]
    fn log_none() -> JfResult<()> {
        async_test(async move {
            let log_writer = Mock::new();
            let mut log_worker = Worker::new();
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
