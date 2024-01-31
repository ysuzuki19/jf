#[cfg(test)]
mod tests;

use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use futures::{stream, StreamExt};
use tokio::sync::Mutex;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{runner::Bunshin, types::JfHandle, Job, Runner},
    jobdef::{Agent, JobdefPool},
    util::{
        error::{IntoJfError, JfResult},
        ReadOnly,
    },
};

#[derive(Clone, serde::Deserialize)]
pub struct SequentialParams {
    pub jobs: Vec<String>,
}

#[derive(Clone)]
pub struct Sequential<LR: LogWriter> {
    jobs: ReadOnly<Vec<Job<LR>>>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JfHandle>>>,
}

impl<LR: LogWriter> Sequential<LR> {
    pub fn new(params: SequentialParams, pool: JobdefPool) -> JfResult<Self> {
        if params.jobs.is_empty() {
            return Err("mode=sequential must have at least one job".into_jf_error());
        }
        let jobs = params
            .jobs
            .into_iter()
            .map(|job_name| pool.build(job_name, Agent::Job))
            .collect::<JfResult<Vec<Job<LR>>>>()?;
        Ok(Self {
            jobs: jobs.into(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Sequential<LR> {
    async fn bunshin(&self) -> Self {
        Self {
            jobs: stream::iter(self.jobs.clone().unwrap().into_iter())
                .then(|j| async move { j.bunshin().await })
                .collect::<Vec<Job<LR>>>()
                .await
                .into(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Sequential<LR> {
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self> {
        let handle: JfHandle = tokio::spawn({
            let ctx = ctx.clone();
            let mut jobs = self.jobs.clone().unwrap();
            let is_cancelled = self.is_cancelled.clone();
            let job = jobs[0]
                .link_cancel(is_cancelled.clone())
                .start(ctx.clone())
                .await?; // start first job immediately

            async move {
                job.join().await?;
                for mut job in jobs.into_iter().skip(1) {
                    //TODO: remove this statement
                    if is_cancelled.load(Ordering::Relaxed) {
                        job.cancel().await?;
                        continue;
                    }
                    job.link_cancel(is_cancelled.clone())
                        .start(ctx.clone())
                        .await?
                        .join()
                        .await?;
                }
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        match self.handle.lock().await.deref() {
            Some(handle) => Ok(handle.is_finished()),
            None => Ok(false), // not started yet
        }
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(self.clone())
    }

    fn link_cancel(&mut self, is_cancelled: Arc<AtomicBool>) -> Self {
        self.is_cancelled = is_cancelled;
        self.clone()
    }
}

impl<LR: LogWriter> From<Sequential<LR>> for Job<LR> {
    fn from(value: Sequential<LR>) -> Self {
        Self::Sequential(value)
    }
}
