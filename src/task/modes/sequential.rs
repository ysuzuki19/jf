use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use super::super::runner::Runner;
use crate::{
    error::CmdResult,
    task::{types::CmdHandle, Task},
    taskdef::{Agent, TaskdefPool},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SequentialParams {
    pub tasks: Vec<String>,
}

#[derive(Clone)]
pub struct Sequential {
    tasks: Vec<Task>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<CmdHandle>>>,
}

impl Sequential {
    pub fn new(params: SequentialParams, pool: TaskdefPool) -> CmdResult<Self> {
        let tasks = params
            .tasks
            .into_iter()
            .map(|task_name| pool.build(task_name, Agent::Task))
            .collect::<CmdResult<Vec<Task>>>()?;
        Ok(Self {
            tasks,
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn run(&self) -> CmdResult<Self> {
        let handle: CmdHandle = tokio::spawn({
            let tasks = self.tasks.clone();
            let is_cancelled = self.is_cancelled.clone();

            async move {
                for task in tasks {
                    if is_cancelled.load(Ordering::Relaxed) {
                        task.cancel().await?;
                        continue;
                    }
                    task.run().await?;
                    task.wait_with_cancel(is_cancelled.clone()).await?;
                }
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(handle) = self.clone().handle.lock().await.deref_mut() {
            Ok(handle.is_finished())
        } else {
            Ok(true)
        }
    }

    async fn cancel(&self) -> CmdResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            tasks: self.tasks.iter().map(|task| task.bunshin()).collect(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Sequential> for Task {
    fn from(value: Sequential) -> Self {
        Task::Sequential(value)
    }
}
