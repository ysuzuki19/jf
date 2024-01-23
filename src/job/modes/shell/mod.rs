#[cfg(test)]
mod tests;

use crate::{
    error::JfResult,
    job::{Job, Runner},
};

#[derive(Clone, serde::Deserialize)]
pub struct ShellParams {
    pub script: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Shell {
    params: ShellParams,
    command: super::Command,
}

impl Shell {
    pub fn new(params: ShellParams) -> Self {
        let mut args = params.args.clone().unwrap_or_default();
        args.extend(vec!["-c".to_string(), params.script.clone()]);
        let command = super::Command::new(super::CommandParams {
            command: "sh".to_string(),
            args,
        });
        Self { params, command }
    }
}

#[async_trait::async_trait]
impl Runner for Shell {
    async fn start(&self) -> JfResult<Self> {
        self.command.start().await?;
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        self.command.is_finished().await
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.command.cancel().await?;
        Ok(self.clone())
    }

    fn bunshin(&self) -> Self {
        let command = self.command.bunshin();
        Self {
            params: self.params.clone(),
            command,
        }
    }
}

impl From<Shell> for Job {
    fn from(value: Shell) -> Self {
        Self::Shell(value)
    }
}
