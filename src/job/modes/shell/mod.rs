#[cfg(test)]
mod tests;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{runner::*, Job},
    util::{error::JfResult, ReadOnly},
};

#[derive(Clone, serde::Deserialize)]
pub struct ShellParams {
    pub script: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Shell<LR: LogWriter> {
    ctx: Ctx<LR>,
    params: ReadOnly<ShellParams>,
    command: super::Command<LR>,
}

impl<LR: LogWriter> Shell<LR> {
    pub fn new(ctx: Ctx<LR>, params: ShellParams) -> Self {
        let mut args = params.args.clone().unwrap_or_default();
        args.extend(vec!["-c".to_string(), params.script.clone()]);
        let command = super::Command::new(
            ctx.clone(),
            super::CommandParams {
                command: "sh".to_string(),
                args,
            },
        );
        Self {
            ctx,
            params: params.into(),
            command,
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Shell<LR> {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            params: self.params.clone(),
            command: self.command.bunshin().await,
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Checker for Shell<LR> {
    async fn is_finished(&self) -> JfResult<bool> {
        self.command.is_finished().await
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Shell<LR> {
    async fn start(&self) -> JfResult<Self> {
        let mut logger = self.ctx.logger();
        logger.debug("Shell starting...").await?;
        self.command.start().await?;
        logger.debug("Shell started").await?;
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.command.cancel().await?;
        Ok(self.clone())
    }
}

impl<LR: LogWriter> From<Shell<LR>> for Job<LR> {
    fn from(value: Shell<LR>) -> Self {
        Self::Shell(value)
    }
}
