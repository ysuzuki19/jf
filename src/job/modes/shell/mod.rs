#[cfg(test)]
mod tests;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{Job, Runner},
    util::{error::JfResult, ReadOnly},
};

#[derive(Clone, serde::Deserialize)]
pub struct ShellParams {
    pub script: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Shell<LR: LogWriter> {
    params: ReadOnly<ShellParams>,
    command: super::Command<LR>,
}

impl<LR: LogWriter> Shell<LR> {
    pub fn new(params: ShellParams) -> Self {
        let mut args = params.args.clone().unwrap_or_default();
        args.extend(vec!["-c".to_string(), params.script.clone()]);
        let command = super::Command::new(super::CommandParams {
            command: "sh".to_string(),
            args,
        });
        Self {
            params: params.into(),
            command,
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Shell<LR> {
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self> {
        self.command.start(ctx).await?;
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        self.command.is_finished().await
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.command.cancel().await?;
        Ok(self.clone())
    }

    async fn bunshin(&self) -> Self {
        let command = self.command.bunshin().await;
        Self {
            params: self.params.clone(),
            command,
        }
    }
}

impl<LR: LogWriter> From<Shell<LR>> for Job<LR> {
    fn from(value: Shell<LR>) -> Self {
        Self::Shell(value)
    }
}
