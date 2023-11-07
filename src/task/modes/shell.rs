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
    params: Params,
    child: Arc<Mutex<Option<tokio::process::Child>>>,
}

impl Shell {
    pub fn new(params: Params) -> Self {
        Self {
            params,
            child: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl Runner for Shell {
    async fn run(&self) -> CmdResult<Self> {
        let child = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(self.params.script.clone())
            .spawn()?;
        self.child.lock().await.replace(child);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            Ok(child.try_wait()?.is_some())
        } else {
            Ok(true)
        }
    }

    async fn cancel(&self) -> CmdResult<()> {
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

impl From<Shell> for Task {
    fn from(value: Shell) -> Self {
        Task::Shell(value)
    }
}
