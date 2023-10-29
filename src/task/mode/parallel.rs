use std::{ops::DerefMut, sync::Arc};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    common,
    error::{CmdError, CmdResult},
    task::{runner::Runner, Task},
};

type CmdHandle = JoinHandle<CmdResult<()>>;

#[derive(Clone)]
pub struct Parallel {
    tasks: Vec<Task>,
    handles: Arc<Mutex<Option<Vec<CmdHandle>>>>,
}

#[async_trait::async_trait]
impl Runner for Parallel {
    async fn run(&self) -> CmdResult<()> {
        let mut handles = Vec::new();
        for task in self.tasks.clone() {
            let handle: JoinHandle<CmdResult<()>> = tokio::spawn({
                async move {
                    task.run().await?;
                    task.wait().await?;
                    Ok(())
                }
            });
            handles.push(handle);
        }
        self.handles.lock().await.replace(handles);

        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(handles) = self.clone().handles.lock().await.deref_mut() {
            let mut is_finished = true;
            for handle in handles.iter() {
                if !handle.is_finished() {
                    is_finished = false;
                }
            }
            Ok(is_finished)
        } else {
            Ok(true)
        }
    }

    async fn kill(self) -> CmdResult<()> {
        if let Some(handles) = self.handles.lock().await.deref_mut() {
            for handle in handles {
                handle.abort();
            }
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            tasks: self.tasks.iter().map(|task| task.bunshin()).collect(),
            handles: Arc::new(Mutex::new(None)),
        }
    }
}

impl Parallel {
    pub fn new(
        runner_config: crate::config::RunnerConfig,
        bc: common::BuildContext,
    ) -> CmdResult<Self> {
        let tasks = runner_config
            .tasks
            .ok_or_else(|| CmdError::TaskdefMissingField("sequential".into(), "tasks".into()))?
            .into_iter()
            .map(|task_name| bc.build(task_name))
            .collect::<CmdResult<Vec<Task>>>()?;
        Ok(Self {
            tasks,
            handles: Arc::new(Mutex::new(Some(Vec::new()))),
        })
    }
}

impl From<Parallel> for Task {
    fn from(value: Parallel) -> Self {
        Task::Parallel(value)
    }
}
