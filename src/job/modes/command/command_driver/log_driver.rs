use tokio::io::{AsyncBufReadExt, BufReader};

use crate::{
    ctx::Ctx,
    job::JfHandle,
    util::error::{IntoJfError, JfResult},
};

pub(super) struct LogDriver {
    ctx: Ctx,
    handle: Option<JfHandle>,
}

impl LogDriver {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx, handle: None }
    }

    pub fn mount(&mut self, stdout: Option<tokio::process::ChildStdout>) -> JfResult<()> {
        match stdout {
            Some(stdout) => {
                self.start(stdout);
                Ok(())
            }
            None => Err("".into_jf_error()),
        }
    }

    pub fn start(&mut self, stdout: tokio::process::ChildStdout) {
        let handle = tokio::spawn({
            let mut logger = self.ctx.logger();
            async move {
                let mut reader = BufReader::new(stdout).lines();

                while let Some(line) = reader.next_line().await? {
                    logger.info(line).await?;
                }

                Ok(())
            }
        });
        self.handle.replace(handle);
    }

    pub async fn join(&mut self) -> JfResult<()> {
        if let Some(handle) = self.handle.take() {
            handle.await?
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{logging::LoggingMock, util::testutil::*};

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn fail_to_mount() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let ctx = Ctx::async_fixture().await;
                let mut log_driver = LogDriver::new(ctx);
                assert!(log_driver.mount(None).is_err());
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn empty_join() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let ctx = Ctx::async_fixture().await;
                let mut log_driver = LogDriver::new(ctx);
                assert!(log_driver.join().await.is_ok());
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_log_driver() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let logging_mock = LoggingMock::new().await;
                let ctx = Ctx::new(logging_mock.logger);
                let mut log_driver = LogDriver::new(ctx);
                assert_eq!(logging_mock.log_writer.lines().len(), 0);

                let mut child = tokio::process::Command::new("echo")
                    .arg("hello")
                    .stdout(std::process::Stdio::piped())
                    .spawn()?;
                log_driver.mount(child.stdout.take())?;
                child.wait().await?;
                log_driver.join().await?;
                assert_eq!(logging_mock.log_writer.lines(), vec!["[I] hello"]);

                let mut child = tokio::process::Command::new("echo")
                    .arg("hello")
                    .stdout(std::process::Stdio::piped())
                    .spawn()?;
                log_driver.mount(child.stdout.take())?;
                child.wait().await?;
                log_driver.join().await?;
                assert_eq!(
                    logging_mock.log_writer.lines(),
                    vec!["[I] hello", "[I] hello"]
                );
                Ok(())
            },
        )
    }
}
