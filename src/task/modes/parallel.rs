use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{
    common::BuildContext,
    error::CmdResult,
    task::{runner::Runner, types::CmdHandle, Task},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Params {
    pub tasks: Vec<String>,
}

#[derive(Clone)]
pub struct Parallel {
    tasks: Vec<Task>,
    handles: Arc<Mutex<Option<Vec<CmdHandle>>>>,
    stop_signal: Arc<AtomicBool>,
}

impl Parallel {
    pub fn new(params: Params, bc: BuildContext) -> CmdResult<Self> {
        let tasks = params
            .tasks
            .into_iter()
            .map(|task_name| bc.build(task_name))
            .collect::<CmdResult<Vec<Task>>>()?;
        Ok(Self {
            tasks,
            handles: Arc::new(Mutex::new(Some(Vec::new()))),
            stop_signal: Arc::new(AtomicBool::new(false)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Parallel {
    async fn run(&self) -> CmdResult<Self> {
        let mut handles = Vec::new();
        for task in self.tasks.clone() {
            let handle: CmdHandle = tokio::spawn({
                let stop_signal = self.stop_signal.clone();
                async move {
                    task.run().await?;
                    loop {
                        if stop_signal.load(Ordering::SeqCst) {
                            task.kill().await?;
                            break;
                        }

                        if task.is_finished().await? {
                            break;
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                    Ok(())
                }
            });
            handles.push(handle);
        }
        self.handles.lock().await.replace(handles);

        Ok(self.clone())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
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

    async fn kill(self) -> CmdResult<()> {
        self.stop_signal.store(true, Ordering::SeqCst);
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
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl From<Parallel> for Task {
    fn from(value: Parallel) -> Self {
        Task::Parallel(value)
    }
}
