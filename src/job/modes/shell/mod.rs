#[cfg(test)]
mod tests;

use crate::{
    ctx::Ctx,
    job::{canceller::Canceller, runner::*, Job},
    util::{error::JfResult, ReadOnly},
};

#[derive(Clone, serde::Deserialize)]
pub struct ShellParams {
    pub script: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Shell {
    ctx: Ctx,
    params: ReadOnly<ShellParams>,
    command: super::Command,
}

impl Shell {
    pub fn new(ctx: Ctx, params: ShellParams) -> Self {
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
impl Bunshin for Shell {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            params: self.params.clone(),
            command: self.command.bunshin().await,
        }
    }
}

#[async_trait::async_trait]
impl Checker for Shell {
    async fn is_finished(&self) -> JfResult<bool> {
        self.command.is_finished().await
    }
}

#[async_trait::async_trait]
impl Runner for Shell {
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

    fn set_canceller(&mut self, canceller: Canceller) -> Self {
        self.command.set_canceller(canceller);
        self.clone()
    }
}

impl From<Shell> for Job {
    fn from(value: Shell) -> Self {
        Self::Shell(value)
    }
}
