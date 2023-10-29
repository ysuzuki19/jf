use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::{sync::Mutex, task::JoinHandle};

use super::super::runner::Runner;
use crate::{
    common::BuildContext,
    error::CmdResult,
    task::{types::CmdHandle, Task},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Params {
    pub tasks: Vec<String>,
}

#[derive(Clone)]
pub struct Sequential {
    tasks: Vec<Task>,
    running_task: Arc<Mutex<Option<Task>>>,
    stop_signal: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<CmdHandle>>>,
}

impl Sequential {
    pub fn new(params: Params, bc: BuildContext) -> CmdResult<Self> {
        let tasks = params
            .tasks
            .into_iter()
            .map(|task_name| bc.build(task_name))
            .collect::<CmdResult<Vec<Task>>>()?;
        Ok(Self {
            tasks,
            running_task: Arc::new(Mutex::new(None)),
            stop_signal: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn run(&self) -> CmdResult<()> {
        let handle: JoinHandle<CmdResult<()>> = tokio::spawn({
            let tasks = self.tasks.clone();
            let running_task = self.running_task.clone();
            let stop_signal = self.stop_signal.clone();

            async move {
                for task in tasks {
                    if stop_signal.load(Ordering::Relaxed) {
                        break;
                    }
                    task.run().await?;
                    running_task.lock().await.replace(task.clone());
                    task.wait().await?;
                }
                running_task.lock().await.take();
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(handle) = self.clone().handle.lock().await.deref_mut() {
            Ok(handle.is_finished())
        } else {
            Ok(true)
        }
    }

    async fn kill(self) -> CmdResult<()> {
        self.stop_signal.store(true, Ordering::Relaxed);

        if let Some(running_task) = self.running_task.lock().await.take() {
            running_task.kill().await?;
        }
        if let Some(handle) = self.handle.lock().await.deref_mut() {
            handle.abort();
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            tasks: self.tasks.iter().map(|task| task.bunshin()).collect(),
            running_task: Arc::new(Mutex::new(None)),
            stop_signal: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Sequential> for Task {
    fn from(value: Sequential) -> Self {
        Task::Sequential(value)
    }
}
