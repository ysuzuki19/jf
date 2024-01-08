use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    error::JfResult,
    job::{Job, Runner},
};

#[derive(Clone, serde::Deserialize)]
pub struct CommandParams {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Clone)]
pub struct Command {
    params: CommandParams,
    child: Arc<Mutex<Option<tokio::process::Child>>>,
}

impl Command {
    pub fn new(params: CommandParams) -> Self {
        Self {
            params,
            child: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl Runner for Command {
    async fn start(&self) -> JfResult<Self> {
        let mut cmd = tokio::process::Command::new(self.params.command.clone());
        cmd.args(self.params.args.clone());
        self.child.lock().await.replace(cmd.spawn()?);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        match self.child.lock().await.deref_mut() {
            Some(ref mut child) => Ok(child.try_wait()?.is_some()),
            None => Ok(false), // not yet started
        }
    }

    async fn cancel(&self) -> JfResult<()> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            if let Err(e) = child.kill().await {
                match e.kind() {
                    std::io::ErrorKind::InvalidInput => {}
                    _ => return Err(e.into()),
                }
            }
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            params: self.params.clone(),
            child: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Command> for Job {
    fn from(value: Command) -> Self {
        Self::Command(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::async_test;

    use super::*;

    #[cfg_attr(coverage, coverage(off))]
    fn test_command_factory() -> Command {
        Command::new(CommandParams {
            command: String::from("sleep"),
            args: vec![String::from("1")],
        })
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn run_without_blocking() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let command = test_command_factory();
                command.start().await?;
                assert!(!command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn wait() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let command = test_command_factory();
                command.start().await?;
                command.wait().await?;
                assert!(command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cancel() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let command = test_command_factory();
                command.start().await?.cancel().await?;
                assert!(command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn bunshin() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let command = test_command_factory().bunshin();
                command.start().await?.cancel().await?;
                assert!(command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn is_finished_not_yet_started() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let command = test_command_factory();
                assert!(!command.is_finished().await?);
                Ok(())
            },
        )
    }
}
