#[cfg(test)]
mod tests;
mod watcher;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::Mutex;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{runner::*, types::JfHandle, Job},
    jobdef::{Agent, JobdefPool},
    util::{error::JfResult, ReadOnly},
};

#[derive(Clone, serde::Deserialize)]
pub struct WatchParams {
    pub job: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch<LR: LogWriter> {
    job: Box<ReadOnly<Job<LR>>>,
    running_job: Arc<Mutex<Option<Job<LR>>>>,
    watch_list: Vec<String>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JfHandle>>>,
}

impl<LR: LogWriter> Watch<LR> {
    pub fn new(params: WatchParams, pool: JobdefPool) -> JfResult<Self> {
        let job = pool.build(params.job, Agent::Job)?;
        Ok(Self {
            job: Box::new(job.clone().into()),
            running_job: Arc::new(Mutex::new(None)),
            watch_list: params.watch_list,
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Watch<LR> {
    async fn bunshin(&self) -> Self {
        Self {
            job: Box::new(self.job.read().bunshin().await.into()),
            running_job: Arc::new(Mutex::new(None)),
            watch_list: self.watch_list.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Checker for Watch<LR> {
    async fn is_finished(&self) -> JfResult<bool> {
        let is_finished = self.is_cancelled.load(Ordering::Relaxed);
        Ok(is_finished)
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Watch<LR> {
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self> {
        let handle = tokio::spawn({
            let watcher = watcher::JfWatcher::new(&self.watch_list, self.is_cancelled.clone())?;
            let ctx = ctx.clone();
            let job = self.job.clone();
            let running_job = self.running_job.clone();
            let is_cancelled = self.is_cancelled.clone();
            running_job.lock().await.replace(
                job.read()
                    .bunshin()
                    .await
                    .link_cancel(is_cancelled.clone())
                    .start(ctx.clone())
                    .await?,
            );

            async move {
                loop {
                    watcher.wait()?;

                    if let Some(running_job) = running_job.lock().await.take() {
                        running_job.cancel().await?;
                    }
                    if is_cancelled.load(Ordering::Relaxed) {
                        break;
                    }

                    running_job
                        .lock()
                        .await
                        .replace(job.read().bunshin().await.start(ctx.clone()).await?);
                }
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        if let Some(running_job) = self.running_job.lock().await.take() {
            running_job.cancel().await?;
        }
        Ok(self.clone())
    }

    fn link_cancel(&mut self, is_cancelled: Arc<AtomicBool>) -> Self {
        self.is_cancelled = is_cancelled;
        self.clone()
    }
}

impl<LR: LogWriter> From<Watch<LR>> for Job<LR> {
    fn from(value: Watch<LR>) -> Self {
        Self::Watch(value)
    }
}
