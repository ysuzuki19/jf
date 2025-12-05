// SPDX-License-Identifier: MPL-2.0
mod log_driver;

use crate::{
    ctx::Ctx,
    job::join_status::JoinStatus,
    util::error::{IntoJfError, JfResult},
};

pub struct CommandDriver {
    child: tokio::process::Child,
    log_driver: log_driver::LogDriver,
}

impl CommandDriver {
    pub async fn spawn(
        ctx: Ctx,
        command: &String,
        args: &Vec<String>,
        env: &std::collections::HashMap<String, String>,
    ) -> JfResult<Self> {
        let mut cmd = tokio::process::Command::new(command);
        cmd.args(args);
        cmd.envs(env);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn()?;
        let mut log_driver = log_driver::LogDriver::new(ctx);

        match log_driver.mount(child.stdout.take(), child.stderr.take()) {
            Ok(_) => Ok(Self { child, log_driver }),
            Err(_) => {
                child.kill().await?;
                Err(format!("Failed to handle stdout for {0}", command.to_owned()).into_jf_error())
            }
        }
    }

    #[allow(dead_code)]
    pub async fn is_finished(&mut self) -> JfResult<bool> {
        Ok(self.child.try_wait()?.is_some())
    }

    pub async fn cancel(&mut self) -> JfResult<()> {
        if let Err(e) = self.child.kill().await {
            match e.kind() {
                std::io::ErrorKind::InvalidInput => {}
                _ => return Err(e.into()),
            }
        }
        // self.log_driver.join().await?;
        Ok(())
    }

    pub async fn join(&mut self) -> JfResult<JoinStatus> {
        let status = self.child.wait().await?;
        self.log_driver.join().await?;
        match status.success() {
            true => Ok(JoinStatus::Succeed),
            false => Ok(JoinStatus::Failed),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::{async_test, AsyncFixture};

    use super::*;

    #[test]
    #[coverage(off)]
    fn spawn() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let command = "echo".to_owned();
                let args = vec!["hello".to_owned()];
                let env = std::collections::HashMap::new();
                let ctx = Ctx::async_fixture().await;
                let mut driver = CommandDriver::spawn(ctx, &command, &args, &env).await?;
                assert!(driver.join().await?.is_succeed());
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn cancel() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let command = "sleep".to_owned();
                let args = vec!["10".to_owned()];
                let env = std::collections::HashMap::new();
                let ctx = Ctx::async_fixture().await;
                let mut driver = CommandDriver::spawn(ctx, &command, &args, &env).await?;
                driver.cancel().await?;
                assert!(driver.join().await?.is_failed());
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn failed() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let command = "false".to_owned();
                let env = std::collections::HashMap::new();
                let ctx = Ctx::async_fixture().await;
                let mut driver = CommandDriver::spawn(ctx, &command, &vec![], &env).await?;
                assert!(driver.join().await?.is_failed());
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn spawn_with_env() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async {
                let command = "sh".to_owned();
                let args = vec!["-c".to_owned(), "echo $TEST_VAR".to_owned()];
                let mut env = std::collections::HashMap::new();
                env.insert("TEST_VAR".to_string(), "test_value".to_string());
                let ctx = Ctx::async_fixture().await;
                let mut driver = CommandDriver::spawn(ctx, &command, &args, &env).await?;
                assert!(driver.join().await?.is_succeed());
                Ok(())
            },
        )
    }
}
