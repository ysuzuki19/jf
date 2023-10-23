use crate::error::{CmdError, CmdResult};

use super::mode::{self, Run};
pub use super::Context;

#[derive(Clone)]
pub enum Runner {
    Command(mode::Command),
    Shell(mode::Shell),
    Sequential(mode::Sequential),
    Parallel(mode::Parallel),
    Watch(mode::Watch),
}

impl Runner {
    pub fn new(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let mode = runner_config.mode.clone().unwrap_or("command".to_string());
        match mode.as_str() {
            "command" => Ok(mode::Command::from_config(runner_config)?.into()),
            "shell" => Ok(mode::Shell::from_config(runner_config)?.into()),
            "sequential" => Ok(mode::Sequential::from_config(runner_config)?.into()),
            "parallel" => Ok(mode::Parallel::from_config(runner_config)?.into()),
            "watch" => Ok(mode::Watch::from_config(runner_config)?.into()),
            _ => Err(CmdError::Custom(format!("Unknown mode: {}", mode))),
        }
    }

    #[async_recursion::async_recursion]
    pub async fn run(self, ctx: Context) -> CmdResult<()> {
        match self {
            Self::Command(mut command) => command.run(ctx).await,
            Self::Shell(mut shell) => {
                shell.run(ctx).await?;
                Ok(())
            }
            Self::Sequential(mut sequential) => {
                sequential.run(ctx).await?;
                Ok(())
            }
            Self::Parallel(mut parallel) => {
                parallel.run(ctx).await?;
                Ok(())
            }
            Self::Watch(mut watch) => {
                watch.run(ctx).await?;
                Ok(())
            }
        }
    }

    pub async fn kill(self) -> CmdResult<()> {
        match self {
            Runner::Command(r) => r.kill().await,
            Runner::Shell(r) => r.kill().await,
            Runner::Sequential(r) => r.kill().await,
            Runner::Parallel(r) => r.kill().await,
            Runner::Watch(r) => r.kill().await,
        }
    }
}
