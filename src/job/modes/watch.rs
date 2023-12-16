use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use super::super::runner::Runner;
use crate::{
    error::JfResult,
    job::Job,
    jobdef::{Agent, JobdefPool},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WatchParams {
    pub job: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch {
    job: Box<Job>,
    watch_list: Vec<String>,
    is_cancelled: Arc<AtomicBool>,
}

impl Watch {
    pub fn new(params: WatchParams, pool: JobdefPool) -> JfResult<Self> {
        let job = pool.build(params.job, Agent::Job)?;
        Ok(Self {
            job: Box::new(job),
            watch_list: params.watch_list,
            is_cancelled: Arc::new(AtomicBool::new(false)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Watch {
    async fn run(&self) -> JfResult<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in self.clone().watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        loop {
            let running_job = self.job.bunshin().run().await?;

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

            running_job.cancel().await?;
            if self.is_cancelled.load(Ordering::Relaxed) {
                break;
            }
        }
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        Ok(false)
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            job: Box::new(self.job.bunshin()),
            watch_list: self.watch_list.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl From<Watch> for Job {
    fn from(value: Watch) -> Self {
        Job::Watch(value)
    }
}
