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
    ctx::Ctx,
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
pub struct Command {
    ctx: Ctx,
    params: ReadOnly<CommandParams>,
    command_driver: Arc<Mutex<Option<CommandDriver>>>,
    is_cancelled: Arc<AtomicBool>,
}

impl Command {
    pub fn new(ctx: Ctx, params: CommandParams) -> Self {
        Self {
            ctx,
            params: params.into(),
            command_driver: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl Bunshin for Command {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            params: self.params.clone(),
            command_driver: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl Checker for Command {
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
impl Runner for Command {
    async fn start(&self) -> JfResult<Self> {
        let mut logger = self.ctx.logger();
        logger.debug("Command starting...").await?;
        let cd = CommandDriver::spawn(
            self.ctx.clone(),
            &self.params.read().command,
            &self.params.read().args,
        )
        .await?;
        self.command_driver.lock().await.replace(cd);
        logger.debug("Command started").await?;
        logger.force("").await?; //FIXME: force flush
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        if let Some(command_driver) = self.command_driver.lock().await.deref_mut() {
            command_driver.cancel().await?;
        }
        Ok(self.clone())
    }

    async fn pre_join(&self) -> JfResult<()> {
        self.ctx.logger().force("").await?; //FIXME: force flush
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

impl From<Command> for Job {
    fn from(value: Command) -> Self {
        Self::Command(value)
    }
}
