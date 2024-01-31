mod command_driver;
#[cfg(test)]
mod tests;

use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{runner::*, Job},
    util::{error::JfResult, ReadOnly},
};

use self::command_driver::CommandDriver;

#[derive(Clone, serde::Deserialize)]
pub struct CommandParams {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Clone)]
pub struct Command<LR: LogWriter> {
    params: ReadOnly<CommandParams>,
    command_driver: Arc<Mutex<Option<CommandDriver<LR>>>>,
    is_cancelled: Arc<AtomicBool>,
}

impl<LR: LogWriter> Command<LR> {
    pub fn new(params: CommandParams) -> Self {
        Self {
            params: params.into(),
            command_driver: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Command<LR> {
    async fn bunshin(&self) -> Self {
        Self {
            params: self.params.clone(),
            command_driver: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Checker for Command<LR> {
    async fn is_finished(&self) -> JfResult<bool> {
        match self.command_driver.lock().await.deref_mut() {
            Some(ref mut cd) => Ok(cd.is_finished().await?),
            None => Ok(false), // not yet started
        }
    }

    fn is_cancelled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Command<LR> {
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self> {
        let cd = CommandDriver::spawn(ctx, &self.params.read().command, &self.params.read().args)
            .await?;
        self.command_driver.lock().await.replace(cd);

        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        if let Some(command_driver) = self.command_driver.lock().await.deref_mut() {
            command_driver.cancel().await?;
        }
        Ok(self.clone())
    }

    async fn pre_join(&self) -> JfResult<()> {
        if let Some(command_driver) = self.command_driver.lock().await.deref_mut() {
            command_driver.join().await?;
        }
        Ok(())
    }

    fn link_cancel(&mut self, is_cancelled: Arc<AtomicBool>) -> Self {
        self.is_cancelled = is_cancelled;
        self.clone()
    }
}

impl<LR: LogWriter> From<Command<LR>> for Job<LR> {
    fn from(value: Command<LR>) -> Self {
        Self::Command(value)
    }
}
