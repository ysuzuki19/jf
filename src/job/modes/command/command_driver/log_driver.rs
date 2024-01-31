use std::sync::Arc;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::Mutex,
};

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::JfHandle,
    util::error::{IntoJfError, JfResult},
};

pub(super) struct LogDriver<LR: LogWriter> {
    ctx: Arc<Mutex<Ctx<LR>>>,
    handle: Option<JfHandle>,
}

impl<LR: LogWriter> LogDriver<LR> {
    pub fn new(ctx: Ctx<LR>) -> Self {
        let ctx = Arc::new(Mutex::new(ctx));
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
            let ctx = self.ctx.clone();
            async move {
                let mut reader = BufReader::new(stdout).lines();

                while let Some(line) = reader.next_line().await? {
                    ctx.lock().await.logger.info(line).await?;
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
    use crate::util::testutil::*;

    use super::*;

    impl Fixture for LogDriver<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Self::new(Fixture::fixture())
        }
    }

    impl LogDriver<MockLogWriter> {
        async fn log_writer(&self) -> MockLogWriter {
            self.ctx.lock().await.logger.log_writer().clone()
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn fail_to_mount() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let ctx = Fixture::fixture();
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
                let ctx = Fixture::fixture();
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
                let ctx = Fixture::fixture();
                let mut log_driver = LogDriver::new(ctx);
                assert_eq!(log_driver.log_writer().await.lines.len(), 0);

                let mut child = tokio::process::Command::new("echo")
                    .arg("hello")
                    .stdout(std::process::Stdio::piped())
                    .spawn()?;
                log_driver.mount(child.stdout.take())?;
                child.wait().await?;
                log_driver.join().await?;
                assert_eq!(log_driver.log_writer().await.lines.len(), 1);

                let mut child = tokio::process::Command::new("echo")
                    .arg("hello")
                    .stdout(std::process::Stdio::piped())
                    .spawn()?;
                log_driver.mount(child.stdout.take())?;
                child.wait().await?;
                log_driver.join().await?;
                assert_eq!(log_driver.log_writer().await.lines.len(), 2);
                Ok(())
            },
        )
    }
}
