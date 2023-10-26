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
    pub task: Arc<Mutex<Task>>,
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
            self.task.lock().await.run().await?;

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
            self.kill().await?;
        }
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        Ok(false)
    }

    async fn kill(&self) -> CmdResult<()> {
        self.task.lock().await.kill().await
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
            task: Arc::new(Mutex::new(task)),
            watch_list,
        })
    }
}

impl From<Watch> for Task {
    fn from(value: Watch) -> Self {
        Task::Watch(value)
    }
}
