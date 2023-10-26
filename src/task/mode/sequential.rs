use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct Sequential {
    tasks: Vec<Task>,
    running_task: Arc<Mutex<Option<Task>>>,
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn run(&self) -> CmdResult<()> {
        for task in self.tasks.clone() {
            task.run().await?;
        }
        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(runner) = self.clone().running_task.lock().await.deref_mut() {
            runner.is_finished().await
        } else {
            Ok(true)
        }
    }

    async fn kill(&self) -> CmdResult<()> {
        if let Some(runner) = self.running_task.lock().await.deref_mut() {
            runner.kill().await?;
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
            running_task: Arc::new(Mutex::new(None)),
        })
    }
}

impl From<Sequential> for Task {
    fn from(value: Sequential) -> Self {
        Task::Sequential(value)
    }
}
