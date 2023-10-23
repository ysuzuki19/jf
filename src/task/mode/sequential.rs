use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::{runner::Runner, Agent, Context},
};

use super::Run;

#[derive(Clone)]
pub struct Sequential {
    tasks: Vec<String>, // runners: Vec<Runner>,
    runner: Option<Arc<Mutex<Runner>>>,
}

#[async_trait::async_trait]
impl Run for Sequential {
    async fn run(&mut self, ctx: Context) -> CmdResult<()> {
        for task in self.tasks.clone() {
            ctx.tasks.get_and_run(task, Agent::Task).await?;
        }
        Ok(())
    }
}

impl Sequential {
    pub fn from_config(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let tasks = runner_config
            .tasks
            .ok_or_else(|| CmdError::TaskdefMissingField("sequential".into(), "tasks".into()))?;
        Ok(Self {
            tasks,
            runner: None,
        })
    }

    #[async_recursion::async_recursion]
    pub async fn kill(self) -> CmdResult<()> {
        if let Some(runner) = self.runner {
            runner.lock().await.clone().kill().await?;
        }
        Ok(())
    }
}

impl From<Sequential> for Runner {
    fn from(value: Sequential) -> Self {
        Runner::Sequential(value)
    }
}
