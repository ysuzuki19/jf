use std::{ops::DerefMut, sync::Arc};

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{error::CmdResult, task::Task};

use super::super::runner::Runner;

#[derive(Debug, Clone, Deserialize)]
pub struct Params {
    command: String,
    args: Vec<String>,
}

#[derive(Clone)]
pub struct Command {
    params: Params,
    child: Arc<Mutex<Option<tokio::process::Child>>>,
}

impl Command {
    pub fn new(params: Params) -> Self {
        Self {
            params,
            child: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl Runner for Command {
    async fn run(&self) -> CmdResult<()> {
        let mut cmd = tokio::process::Command::new(self.params.command.clone());
        cmd.args(self.params.args.clone());
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
            params: self.params.clone(),
            child: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Command> for Task {
    fn from(value: Command) -> Self {
        Task::Command(value)
    }
}
