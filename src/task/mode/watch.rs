use std::sync::Arc;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::Mutex;

use crate::{
    common,
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct Watch {
    task: Box<Task>,
    running_task: Arc<Mutex<Option<Task>>>,
    watch_list: Vec<String>,
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
            let task = self.task.bunshin();
            task.run().await?;
            self.running_task.lock().await.replace(task);

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
                running_task.kill().await?;
            }
        }
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        Ok(false)
    }

    async fn kill(self) -> CmdResult<()> {
        if let Some(running_task) = self.running_task.lock().await.take() {
            running_task.kill().await?;
        }
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            task: Box::new(self.task.bunshin()),
            running_task: Arc::new(Mutex::new(None)),
            watch_list: self.watch_list.clone(),
        }
    }
}

impl Watch {
    pub fn new(
        runner_config: crate::config::RunnerConfig,
        bc: common::BuildContext,
    ) -> CmdResult<Self> {
        let task_name = runner_config
            .task
            .ok_or_else(|| CmdError::TaskdefMissingField("watch".into(), "task".into()))?;
        let task = bc.build(task_name)?;
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
