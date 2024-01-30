mod command_driver;
#[cfg(test)]
mod tests;

use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{Job, Runner},
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
}

impl<LR: LogWriter> Command<LR> {
    pub fn new(params: CommandParams) -> Self {
        Self {
            params: params.into(),
            command_driver: Arc::new(Mutex::new(None)),
        }
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

    async fn is_finished(&self) -> JfResult<bool> {
        match self.command_driver.lock().await.deref_mut() {
            Some(ref mut cd) => Ok(cd.is_finished().await?),
            None => Ok(false), // not yet started
        }
    }

    async fn cancel(&self) -> JfResult<Self> {
        if let Some(command_driver) = self.command_driver.lock().await.deref_mut() {
            command_driver.cancel().await?;
        }
        Ok(self.clone())
    }

    fn bunshin(&self) -> Self {
        Self {
            params: self.params.clone(),
            command_driver: Arc::new(Mutex::new(None)),
        }
    }
}

impl<LR: LogWriter> From<Command<LR>> for Job<LR> {
    fn from(value: Command<LR>) -> Self {
        Self::Command(value)
    }
}
