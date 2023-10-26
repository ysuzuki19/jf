use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct Shell {
    pub script: String,
    child: Arc<Mutex<Option<tokio::process::Child>>>,
}

#[async_trait::async_trait]
impl Runner for Shell {
    async fn run(&self) -> CmdResult<()> {
        let mut cmd = tokio::process::Command::new("sh");
        cmd.arg("-c").arg(self.script.clone());
        let child = cmd.spawn()?;
        self.child.lock().await.replace(child);
        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            Ok(child.try_wait()?.is_some())
        } else {
            Ok(true)
        }
    }

    async fn kill(&self) -> CmdResult<()> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            child.kill().await?;
        }
        Ok(())
    }
}

impl Shell {
    pub fn new(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let script = runner_config
            .script
            .ok_or_else(|| CmdError::TaskdefMissingField("shell".into(), "script".into()))?;
        Ok(Self {
            script,
            child: Arc::new(Mutex::new(None)),
        })
    }
}

impl From<Shell> for Task {
    fn from(value: Shell) -> Self {
        Task::Shell(value)
    }
}
