use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

type CmdHandle = JoinHandle<CmdResult<()>>;

#[derive(Clone)]
pub struct Sequential {
    tasks: Vec<Task>,
    running_task: Arc<Mutex<Option<Task>>>,
    is_running: Arc<AtomicBool>,
    stop_signal: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<CmdHandle>>>,
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn run(&self) -> CmdResult<()> {
        self.stop_signal.store(false, Ordering::Relaxed);

        if self
            .is_running
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            let handle: JoinHandle<CmdResult<()>> = tokio::spawn({
                let tasks = self.tasks.clone();
                let running_task = self.running_task.clone();
                let stop_signal = self.stop_signal.clone();

                async move {
                    println!("Start to run sequential tasks");
                    for task in tasks {
                        if stop_signal.load(Ordering::Relaxed) {
                            println!("Stop signal received");
                            break;
                        }
                        println!("Run next sequential task");
                        task.run().await?;
                        running_task.lock().await.replace(task.clone());
                        task.wait().await?;
                    }
                    Ok(())
                }
            });
            self.handle.lock().await.replace(handle);
        }
        Ok(())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        if let Some(handle) = self.clone().handle.lock().await.deref_mut() {
            Ok(handle.is_finished())
        } else {
            Ok(true)
        }
    }

    async fn kill(&self) -> CmdResult<()> {
        self.stop_signal.store(true, Ordering::Relaxed);
        if self
            .is_running
            .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            if let Some(running_task) = self.running_task.lock().await.deref_mut() {
                let _ = running_task.kill().await;
            }
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
            is_running: Arc::new(AtomicBool::new(false)),
            stop_signal: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

impl From<Sequential> for Task {
    fn from(value: Sequential) -> Self {
        Task::Sequential(value)
    }
}
