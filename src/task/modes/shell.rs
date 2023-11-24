use super::super::runner::Runner;
use crate::{error::CmdResult, task::Task};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellParams {
    pub script: String,
}

#[derive(Clone)]
pub struct Shell {
    params: ShellParams,
    command: super::Command,
}

impl Shell {
    pub fn new(params: ShellParams) -> Self {
        let command = super::Command::new(super::CommandParams {
            command: "sh".to_string(),
            args: vec!["-c".to_string(), params.script.clone()],
        });
        Self { params, command }
    }
}

#[async_trait::async_trait]
impl Runner for Shell {
    async fn run(&self) -> CmdResult<Self> {
        self.command.run().await?;
        Ok(self.clone())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        self.command.is_finished().await
    }

    async fn cancel(&self) -> CmdResult<()> {
        self.command.cancel().await?;
        Ok(())
    }

    fn bunshin(&self) -> Self {
        let command = self.command.bunshin();
        Self {
            params: self.params.clone(),
            command,
        }
    }
}

impl From<Shell> for Task {
    fn from(value: Shell) -> Self {
        Task::Shell(value)
    }
}
