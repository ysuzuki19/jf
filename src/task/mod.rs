pub mod modes;
mod runner;
mod types;

use crate::{common::BuildContext, config::TaskConfig, error::CmdResult};

pub use self::runner::Runner;

#[derive(Clone)]
pub enum Task {
    Command(modes::Command),
    Shell(modes::Shell),
    Sequential(modes::Sequential),
    Parallel(modes::Parallel),
    Watch(modes::Watch),
}

impl Task {
    pub fn new(config: TaskConfig, bc: BuildContext) -> CmdResult<Self> {
        Ok(match config {
            TaskConfig::Command(c) => modes::Command::new(c.params).into(),
            TaskConfig::Shell(c) => modes::Shell::new(c.params).into(),
            TaskConfig::Sequential(c) => modes::Sequential::new(c.params, bc)?.into(),
            TaskConfig::Parallel(c) => modes::Parallel::new(c.params, bc)?.into(),
            TaskConfig::Watch(c) => modes::Watch::new(c.params, bc)?.into(),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Task {
    async fn run(&self) -> CmdResult<Self> {
        Ok(match self.clone() {
            Self::Command(command) => command.run().await?.into(),
            Self::Shell(shell) => shell.run().await?.into(),
            Self::Sequential(sequential) => sequential.run().await?.into(),
            Self::Parallel(parallel) => parallel.run().await?.into(),
            Self::Watch(watch) => watch.run().await?.into(),
        })
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
