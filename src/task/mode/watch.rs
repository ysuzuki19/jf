use std::sync::Arc;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    error::{CmdError, CmdResult},
    task::{runner::Runner, Agent, Context},
};

use super::Run;

#[derive(Clone)]
pub struct Watch {
    pub task: String,
    pub watch_list: Vec<String>,
    child: Option<Arc<Mutex<tokio::process::Child>>>,
}

#[async_trait::async_trait]
impl Run for Watch {
    async fn run(&mut self, ctx: Context) -> CmdResult<()> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in self.clone().watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        loop {
            let handle: JoinHandle<CmdResult<()>> = tokio::spawn({
                let ctx = ctx.clone();
                let task = self.task.clone();

                async move {
                    let _ = ctx.tasks.get(&task)?.run(ctx.clone(), Agent::Task).await;
                    Ok(())
                }
            });

            let e = rx.recv()?;
            handle.abort();
            self.clone().kill().await?;
            println!("{:?}", e);
        }
    }
}

impl Watch {
    pub fn from_config(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let task = runner_config
            .task
            .ok_or_else(|| CmdError::TaskdefMissingField("watch".into(), "task".into()))?;
        let watch_list = runner_config
            .watch_list
            .ok_or_else(|| CmdError::TaskdefMissingField("watch".into(), "watch_list".into()))?;
        Ok(Self {
            task,
            watch_list,
            child: None,
        })
    }

    pub async fn kill(self) -> CmdResult<()> {
        if let Some(child) = self.child {
            child.lock().await.kill().await?;
        }
        todo!();
    }
}

impl From<Watch> for Runner {
    fn from(value: Watch) -> Self {
        Runner::Watch(value)
    }
}
