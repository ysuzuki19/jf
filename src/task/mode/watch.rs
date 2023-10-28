use std::sync::Arc;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct Watch {
    // pub task: Arc<Mutex<Task>>,
    pub task: Box<Task>,
    pub running_task: Arc<Mutex<Option<Box<Task>>>>,
    pub watch_list: Vec<String>,
}

#[async_trait::async_trait]
impl Runner for Watch {
    async fn run(&self) -> CmdResult<()> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in self.clone().watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        loop {
            let task = self.task.clone();
            task.run().await?;
            self.running_task.lock().await.replace(task);
            // self.running_task.lock().await.run().await?;

            loop {
                match rx.recv()??.kind {
                    notify::EventKind::Modify(_) => {
                        break;
                    }
                    notify::EventKind::Create(_) => {
                        break;
                    }
                    notify::EventKind::Remove(_) => {
                        break;
                    }
                    _ => {}
                }
            }

            if let Some(running_task) = self.running_task.lock().await.take() {
                println!("running_task is Some");
                running_task.kill().await?;
            } else {
                println!("running_task is None");
            }
        }
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        Ok(false)
    }

    async fn kill(&self) -> CmdResult<()> {
        if let Some(running_task) = self.running_task.lock().await.take() {
            running_task.kill().await?;
        }
        Ok(())
        // self.running_task.lock().await.kill().await
    }
}

impl Watch {
    pub fn new(
        runner_config: crate::config::RunnerConfig,
        ctx: crate::taskdef::context::Context,
    ) -> CmdResult<Self> {
        let task_name = runner_config
            .task
            .ok_or_else(|| CmdError::TaskdefMissingField("watch".into(), "task".into()))?;
        let task = ctx.build(task_name)?;
        let watch_list = runner_config
            .watch_list
            .ok_or_else(|| CmdError::TaskdefMissingField("watch".into(), "watch_list".into()))?;
        Ok(Self {
            task: Box::new(task),
            running_task: Arc::new(Mutex::new(None)),
            watch_list,
        })
    }
}

impl From<Watch> for Task {
    fn from(value: Watch) -> Self {
        Task::Watch(value)
    }
}
