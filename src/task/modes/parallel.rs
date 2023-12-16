use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{
    error::JfResult,
    task::{runner::Runner, types::JfHandle, Task},
    taskdef::{Agent, TaskdefPool},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ParallelParams {
    pub tasks: Vec<String>,
}

#[derive(Clone)]
pub struct Parallel {
    tasks: Vec<Task>,
    handles: Arc<Mutex<Option<Vec<JfHandle>>>>,
    is_cancelled: Arc<AtomicBool>,
}

impl Parallel {
    pub fn new(params: ParallelParams, pool: TaskdefPool) -> JfResult<Self> {
        let tasks = params
            .tasks
            .into_iter()
            .map(|task_name| pool.build(task_name, Agent::Task))
            .collect::<JfResult<Vec<Task>>>()?;
        Ok(Self {
            tasks,
            handles: Arc::new(Mutex::new(Some(Vec::new()))),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Parallel {
    async fn run(&self) -> JfResult<Self> {
        let mut handles = Vec::new();
        for task in self.tasks.clone() {
            let handle: JfHandle = tokio::spawn({
                let is_cancelled = self.is_cancelled.clone();
                async move {
                    task.run().await?;
                    task.wait_with_cancel(is_cancelled).await?;
                    Ok(())
                }
            });
            handles.push(handle);
        }
        self.handles.lock().await.replace(handles);

        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        if let Some(handles) = self.clone().handles.lock().await.deref_mut() {
            if handles.iter().all(|h| h.is_finished()) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::SeqCst);
        if let Some(handles) = self.handles.lock().await.deref_mut() {
            for handle in handles {
                let _ = handle.await?;
            }
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            tasks: self.tasks.iter().map(|task| task.bunshin()).collect(),
            handles: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl From<Parallel> for Task {
    fn from(value: Parallel) -> Self {
        Task::Parallel(value)
    }
}
