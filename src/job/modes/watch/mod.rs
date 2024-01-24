#[cfg(test)]
mod tests;
mod watcher_container;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use notify::EventKind;
use tokio::sync::Mutex;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    error::JfResult,
    job::{types::JfHandle, Job, Runner},
    jobdef::{Agent, JobdefPool},
};

use self::watcher_container::WatcherContainer;

#[derive(Clone, serde::Deserialize)]
pub struct WatchParams {
    pub job: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch<LR: LogWriter> {
    job: Box<Job<LR>>,
    running_job: Arc<Mutex<Job<LR>>>,
    watch_list: Vec<String>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JfHandle>>>,
    watcher_container: Arc<Mutex<Option<WatcherContainer>>>,
}

impl<LR: LogWriter> Watch<LR> {
    pub fn new(params: WatchParams, pool: JobdefPool) -> JfResult<Self> {
        let job = pool.build(params.job, Agent::Job)?;
        Ok(Self {
            job: Box::new(job.clone()),
            running_job: Arc::new(Mutex::new(job)),
            watch_list: params.watch_list,
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
            watcher_container: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Watch<LR> {
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self> {
        let (container, rx) = WatcherContainer::new(&self.watch_list)?;
        self.watcher_container.lock().await.replace(container);

        let handle = tokio::spawn({
            let ctx = ctx.clone();
            let job = self.job.clone();
            let running_job = self.running_job.clone();
            let is_cancelled = self.is_cancelled.clone();

            async move {
                loop {
                    *(running_job.lock().await) = job.bunshin().start(ctx.clone()).await?;

                    loop {
                        match rx.recv()??.kind {
                            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                                break;
                            }
                            _ => {}
                        }
                    }

                    running_job.lock().await.cancel().await?;
                    if is_cancelled.load(Ordering::Relaxed) {
                        break;
                    }
                }
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        Ok(self.is_cancelled.load(Ordering::Relaxed))
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        self.running_job.lock().await.cancel().await?;
        Ok(self.clone())
    }

    fn bunshin(&self) -> Self {
        Self {
            job: Box::new(self.job.bunshin()),
            running_job: Arc::new(Mutex::new(self.job.bunshin())),
            watch_list: self.watch_list.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
            watcher_container: Arc::new(Mutex::new(None)),
        }
    }
}

impl<LR: LogWriter> From<Watch<LR>> for Job<LR> {
    fn from(value: Watch<LR>) -> Self {
        Self::Watch(value)
    }
}
