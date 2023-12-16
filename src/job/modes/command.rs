use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use super::super::runner::Runner;
use crate::{error::JfResult, job::Job};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommandParams {
    pub command: String,
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
    async fn run(&self) -> JfResult<Self> {
        let mut jf = tokio::process::Command::new(self.params.command.clone());
        jf.args(self.params.args.clone());
        self.child.lock().await.replace(jf.spawn()?);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            Ok(child.try_wait()?.is_some())
        } else {
            Ok(true)
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
        Job::Command(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn run_without_blocking() -> JfResult<()> {
        let command = Command::new(CommandParams {
            command: "sleep".to_string(),
            args: vec!["1".to_string()],
        });
        command.run().await?;
        assert!(!command.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    async fn wait() -> JfResult<()> {
        let command = Command::new(CommandParams {
            command: "sleep".to_string(),
            args: vec!["1".to_string()],
        });
        command.run().await?;
        command.wait().await?;
        assert!(command.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    async fn cancel() -> JfResult<()> {
        let command = Command::new(CommandParams {
            command: "sleep".to_string(),
            args: vec!["1".to_string()],
        });
        command.run().await?.cancel().await?;
        assert!(command.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    async fn bunshin() -> JfResult<()> {
        let command = Command::new(CommandParams {
            command: "sleep".to_string(),
            args: vec!["1".to_string()],
        })
        .bunshin();
        command.run().await?.cancel().await?;
        assert!(command.is_finished().await?);
        Ok(())
    }
}