use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use super::super::runner::Runner;
use crate::{common::BuildContext, error::CmdResult, task::Task};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WatchParams {
    pub task: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch {
    task: Box<Task>,
    watch_list: Vec<String>,
    is_cancelled: Arc<AtomicBool>,
}

impl Watch {
    pub fn new(params: WatchParams, bc: BuildContext) -> CmdResult<Self> {
        let task = bc.build(params.task)?;
        Ok(Self {
            task: Box::new(task),
            watch_list: params.watch_list,
            is_cancelled: Arc::new(AtomicBool::new(false)),
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
            let running_task = self.task.bunshin().run().await?;

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

            running_task.cancel().await?;
            if self.is_cancelled.load(Ordering::Relaxed) {
                break;
            }
        }
        Ok(self.clone())
    }

    async fn is_finished(&self) -> CmdResult<bool> {
        Ok(false)
    }

    async fn cancel(&self) -> CmdResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            task: Box::new(self.task.bunshin()),
            watch_list: self.watch_list.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl From<Watch> for Task {
    fn from(value: Watch) -> Self {
        Task::Watch(value)
    }
}
