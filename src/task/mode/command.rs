use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::{runner::Runner, Context},
};

use super::Run;

#[derive(Clone)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    child: Option<Arc<Mutex<tokio::process::Child>>>,
}

#[async_trait::async_trait]
impl Run for Command {
    async fn run(&mut self, _: Context) -> CmdResult<()> {
        println!(
            "Run Command\"{}\" with Args({:?})",
            self.command.clone(),
            self.args.clone()
        );
        let mut cmd = tokio::process::Command::new(self.command.clone());
        cmd.args(self.args.clone());
        let child = cmd.spawn()?;
        self.child = Some(Arc::new(Mutex::new(child)));
        self.child.clone().unwrap().lock().await.wait().await?;
        Ok(())
    }
}

impl Command {
    pub fn from_config(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let command = runner_config
            .command
            .ok_or_else(|| CmdError::TaskdefMissingField("command".into(), "command".into()))?;
        let args = runner_config.args.unwrap_or_default();
        Ok(Self {
            command,
            args,
            child: None,
        })
    }

    pub async fn kill(self) -> CmdResult<()> {
        if let Some(child) = self.child {
            child.lock().await.kill().await?;
        }
        Ok(())
    }
}

impl From<Command> for Runner {
    fn from(value: Command) -> Self {
        Runner::Command(value)
    }
}
