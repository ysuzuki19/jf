use std::sync::Arc;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::Mutex;

use super::super::runner::Runner;
use crate::{common::BuildContext, error::CmdResult, task::Task};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Params {
    pub task: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch {
    task: Box<Task>,
    watch_list: Vec<String>,
    running_task: Arc<Mutex<Option<Task>>>,
}

impl Watch {
    pub fn new(params: Params, bc: BuildContext) -> CmdResult<Self> {
        let task = bc.build(params.task)?;
        Ok(Self {
            task: Box::new(task),
            running_task: Arc::new(Mutex::new(None)),
            watch_list: params.watch_list,
        })
    }
}

#[async_trait::async_trait]
impl Runner for Watch {
    async fn run(&self) -> CmdResult<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in self.clone().watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        loop {
            let task = self.task.bunshin().run().await?;
            self.running_task.lock().await.replace(task);

            loop {
                match rx.recv()??.kind {
                    notify::EventKind::Modify(_)
                    | notify::EventKind::Create(_)
                    | notify::EventKind::Remove(_) => {
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

impl From<Watch> for Task {
    fn from(value: Watch) -> Self {
        Task::Watch(value)
    }
}
