mod modes;
pub mod runner;
mod types;

use crate::{
    common::BuildContext,
    error::{CmdError, CmdResult},
};

use self::runner::Runner;

#[derive(Clone)]
pub enum Task {
    Command(modes::Command),
    Shell(modes::Shell),
    Sequential(modes::Sequential),
    Parallel(modes::Parallel),
    Watch(modes::Watch),
}

impl Task {
    pub fn new(runner_config: crate::config::RunnerConfig, bc: BuildContext) -> CmdResult<Self> {
        let mode = runner_config.mode.clone().unwrap_or("command".to_string());
        match mode.as_str() {
            "command" => Ok(modes::Command::new(runner_config)?.into()),
            "shell" => Ok(modes::Shell::new(runner_config)?.into()),
            "sequential" => Ok(modes::Sequential::new(runner_config, bc)?.into()),
            "parallel" => Ok(modes::Parallel::new(runner_config, bc)?.into()),
            "watch" => Ok(modes::Watch::new(runner_config, bc)?.into()),
            _ => Err(CmdError::Custom(format!("Unknown mode: {}", mode))),
        }
    }
}

#[async_trait::async_trait]
impl Runner for Task {
    async fn run(&self) -> CmdResult<()> {
        match self.clone() {
            Self::Command(command) => command.run().await?,
            Self::Shell(shell) => shell.run().await?,
            Self::Sequential(sequential) => sequential.run().await?,
            Self::Parallel(parallel) => parallel.run().await?,
            Self::Watch(watch) => watch.run().await?,
        }
        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        match self.clone() {
            Self::Command(command) => command.is_finished().await,
            Self::Shell(shell) => shell.is_finished().await,
            Self::Sequential(sequential) => sequential.is_finished().await,
            Self::Parallel(parallel) => parallel.is_finished().await,
            Self::Watch(watch) => watch.is_finished().await,
        }
    }

    async fn kill(self) -> CmdResult<()> {
        match self.clone() {
            Self::Command(command) => command.kill().await?,
            Self::Shell(shell) => shell.kill().await?,
            Self::Sequential(sequential) => sequential.kill().await?,
            Self::Parallel(parallel) => parallel.kill().await?,
            Self::Watch(watch) => watch.kill().await?,
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        match self.clone() {
            Self::Command(command) => command.bunshin().into(),
            Self::Shell(shell) => shell.bunshin().into(),
            Self::Sequential(sequential) => sequential.bunshin().into(),
            Self::Parallel(parallel) => parallel.bunshin().into(),
            Self::Watch(watch) => watch.bunshin().into(),
        }
    }
}
