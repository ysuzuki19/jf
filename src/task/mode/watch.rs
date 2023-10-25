use std::sync::Arc;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    error::{CmdError, CmdResult},
    task::Task,
};

use super::super::runner::Runner;

#[derive(Clone)]
pub struct Watch {
    pub task: Arc<Mutex<Task>>,
    pub watch_list: Vec<String>,
    child: Option<Arc<Mutex<tokio::process::Child>>>,
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
            let handle: JoinHandle<CmdResult<()>> = tokio::spawn({
                let task = self.task.clone();

                async move {
                    let task = task.lock().await;
                    task.run().await?;
                    task.wait().await?;
                    Ok(())
                }
            });

            loop {
                match rx.recv()? {
                    Ok(notify::Event { kind, .. }) => match kind {
                        notify::EventKind::Modify(_) => {
                            println!("File modified, restarting task...");
                            break;
                        }
                        notify::EventKind::Create(_) => {
                            println!("File created, restarting task...");
                            break;
                        }
                        notify::EventKind::Remove(_) => {
                            println!("File removed, restarting task...");
                            break;
                        }
                        _ => {}
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
            handle.abort();
            self.clone().kill().await?;
        }
    }

    async fn wait(&self) -> CmdResult<()> {
        loop {
            if let Some(child) = self.child.clone() {
                if child.lock().await.try_wait()?.is_some() {
                    break;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        Ok(())
    }

    async fn kill(self) -> CmdResult<()> {
        if let Some(child) = &self.child {
            child.lock().await.kill().await?;
        }
        Ok(())
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
            child: None,
        })
    }
}

impl From<Watch> for Task {
    fn from(value: Watch) -> Self {
        Task::Watch(value)
    }
}
