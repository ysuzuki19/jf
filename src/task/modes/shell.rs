use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use super::super::runner::Runner;
use crate::{error::CmdResult, task::Task};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Params {
    script: String,
}

#[derive(Clone)]
pub struct Shell {
    script: String,
    child: Arc<Mutex<Option<tokio::process::Child>>>,
}

impl Shell {
    pub fn new(params: Params) -> Self {
        let script = params.script;
        Self {
            script,
            child: Arc::new(Mutex::new(None)),
        }
    }
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

    async fn kill(self) -> CmdResult<()> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            child.kill().await?;
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            script: self.script.clone(),
            child: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Shell> for Task {
    fn from(value: Shell) -> Self {
        Task::Shell(value)
    }
}
