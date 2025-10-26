// SPDX-License-Identifier: MPL-2.0
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::StreamExt;

use crate::{
    ctx::Ctx,
    job::{join_status::JoinStatus, JfHandle},
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

    pub fn mount(
        &mut self,
        stdout: Option<tokio::process::ChildStdout>,
        stderr: Option<tokio::process::ChildStderr>,
    ) -> JfResult<()> {
        match (stdout, stderr) {
            (Some(stdout), Some(stderr)) => {
                self.start(stdout, stderr);
                Ok(())
            }
            _ => Err("".into_jf_error()),
        }
    }

    pub fn start(
        &mut self,
        stdout: tokio::process::ChildStdout,
        stderr: tokio::process::ChildStderr,
    ) {
        let handle = tokio::spawn({
            let mut logger = self.ctx.logger();
            async move {
                enum StreamLine {
                    Stdout(String),
                    Stderr(String),
                }

                let stdout = BufReader::new(stdout).lines();
                let stderr = BufReader::new(stderr).lines();

                let stdout = tokio_stream::wrappers::LinesStream::new(stdout)
                    .map(|line| JfResult::Ok(StreamLine::Stdout(line?)));
                let stderr = tokio_stream::wrappers::LinesStream::new(stderr)
                    .map(|line| JfResult::Ok(StreamLine::Stderr(line?)));

                let mut reader = tokio_stream::StreamExt::merge(stdout, stderr);

                while let Some(Ok(line)) = reader.next().await {
                    match line {
                        StreamLine::Stdout(line) => logger.info(line).await?,
                        StreamLine::Stderr(line) => logger.error(line).await?,
                    }
                }

                Ok(JoinStatus::Succeed)
            }
        });
        self.handle.replace(handle);
    }

    pub async fn join(&mut self) -> JfResult<JoinStatus> {
        if let Some(handle) = self.handle.take() {
            handle.await?
        } else {
            Ok(JoinStatus::Failed)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{logging::LoggingMock, util::testutil::*};

    use super::*;

    #[test]
    #[coverage(off)]
    fn fail_to_mount() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let ctx = Ctx::async_fixture().await;
                let mut log_driver = LogDriver::new(ctx);
                assert!(log_driver.mount(None, None).is_err());
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn empty_join() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let ctx = Ctx::async_fixture().await;
                let mut log_driver = LogDriver::new(ctx);
                assert!(log_driver.join().await.is_ok());
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn log_driver() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let logging_mock = LoggingMock::new().await;
                let ctx = Ctx::new(logging_mock.logger, "test", true);
                let mut log_driver = LogDriver::new(ctx);
                assert_eq!(logging_mock.log_writer.lines().len(), 0);

                let mut child = tokio::process::Command::new("echo")
                    .arg("hello")
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()?;
                log_driver.mount(child.stdout.take(), child.stderr.take())?;
                child.wait().await?;
                log_driver.join().await?;
                assert_eq!(logging_mock.log_writer.lines(), vec!["[I] hello"]);

                let mut child = tokio::process::Command::new("echo")
                    .arg("hello")
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()?;
                log_driver.mount(child.stdout.take(), child.stderr.take())?;
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
