use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct Sequential {
    tasks: Vec<Task>,
    running_task: Option<Arc<Mutex<Task>>>,
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn run(&self) -> CmdResult<()> {
        for task in self.tasks.clone() {
            task.run().await?;
        }
        Ok(())
    }

    async fn wait(&self) -> CmdResult<()> {
        //TODO: change to non-blocking
        if let Some(runner) = self.clone().running_task {
            runner.lock().await.wait().await?;
        }
        Ok(())
    }

    async fn kill(self) -> CmdResult<()> {
        if let Some(runner) = self.running_task {
            runner.lock().await.clone().kill().await?;
        }
        Ok(())
    }
}

impl Sequential {
    pub fn new(
        runner_config: crate::config::RunnerConfig,
        ctx: crate::taskdef::context::Context,
    ) -> CmdResult<Self> {
        let tasks = runner_config
            .tasks
            .ok_or_else(|| CmdError::TaskdefMissingField("sequential".into(), "tasks".into()))?
            .into_iter()
            .map(|task_name| ctx.build(task_name))
            .collect::<CmdResult<Vec<Task>>>()?;
        Ok(Self {
            tasks,
            running_task: None,
        })
    }
}

impl From<Sequential> for Task {
    fn from(value: Sequential) -> Self {
        Task::Sequential(value)
    }
}
