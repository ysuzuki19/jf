use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct CommandConfig {
    command: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let command = runner_config
            .command
            .ok_or_else(|| CmdError::TaskdefMissingField("command".into(), "command".into()))?;
        let args = runner_config.args.unwrap_or_default();
        Ok(Self {
            config: CommandConfig { command, args },
            child: Arc::new(Mutex::new(None)),
        })
    }
}

#[derive(Clone)]
pub struct Command {
    config: CommandConfig,
    child: Arc<Mutex<Option<tokio::process::Child>>>,
}

#[async_trait::async_trait]
impl Runner for Command {
    async fn run(&self) -> CmdResult<()> {
        let mut cmd = tokio::process::Command::new(self.config.command.clone());
        cmd.args(self.config.args.clone());
        self.child.lock().await.replace(cmd.spawn()?);
        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            Ok(child.try_wait()?.is_some())
        } else {
            Ok(true)
        }
    }

    async fn kill(self) -> CmdResult<()> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            child.kill().await?;
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            config: self.config.clone(),
            child: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Command> for Task {
    fn from(value: Command) -> Self {
        Task::Command(value)
    }
}
