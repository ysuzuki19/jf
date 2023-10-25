mod mode;
pub mod runner;

use crate::error::{CmdError, CmdResult};

use self::runner::Runner;

#[derive(Clone)]
pub enum Agent {
    Cli,
    Task,
}

#[derive(Clone)]
pub enum Task {
    Command(mode::Command),
    Shell(mode::Shell),
    Sequential(mode::Sequential),
    Parallel(mode::Parallel),
    Watch(mode::Watch),
}

impl Task {
    pub fn new(
        runner_config: crate::config::RunnerConfig,
        ctx: crate::taskdef::context::Context,
    ) -> CmdResult<Self> {
        let mode = runner_config.mode.clone().unwrap_or("command".to_string());
        match mode.as_str() {
            "command" => Ok(mode::Command::new(runner_config)?.into()),
            "shell" => Ok(mode::Shell::new(runner_config)?.into()),
            "sequential" => Ok(mode::Sequential::new(runner_config, ctx)?.into()),
            "parallel" => Ok(mode::Parallel::new(runner_config, ctx)?.into()),
            "watch" => Ok(mode::Watch::new(runner_config, ctx)?.into()),
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

    async fn wait(&self) -> CmdResult<()> {
        match self.clone() {
            Self::Command(command) => command.wait().await?,
            Self::Shell(shell) => shell.wait().await?,
            Self::Sequential(sequential) => sequential.wait().await?,
            Self::Parallel(parallel) => parallel.wait().await?,
            Self::Watch(watch) => watch.wait().await?,
        }
        Ok(())
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
}
