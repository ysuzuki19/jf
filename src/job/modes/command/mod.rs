#[cfg(test)]
mod tests;

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

    async fn cancel(&self) -> JfResult<Self> {
        if let Some(ref mut child) = self.child.lock().await.deref_mut() {
            if let Err(e) = child.kill().await {
                match e.kind() {
                    std::io::ErrorKind::InvalidInput => {}
                    _ => return Err(e.into()),
                }
            }
        }
        Ok(self.clone())
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
