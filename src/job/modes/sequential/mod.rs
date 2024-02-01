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
    ctx::Ctx,
    job::{runner::*, Job},
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
pub struct Sequential {
    ctx: Ctx,
    jobs: ReadOnly<Vec<Job>>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JfHandle>>>,
}

impl Sequential {
    pub fn new(ctx: Ctx, params: SequentialParams, pool: JobdefPool) -> JfResult<Self> {
        if params.jobs.is_empty() {
            return Err("mode=sequential must have at least one job".into_jf_error());
        }
        let jobs = params
            .jobs
            .into_iter()
            .map(|job_name| pool.build(ctx.clone(), job_name, Agent::Job))
            .collect::<JfResult<Vec<Job>>>()?;
        Ok(Self {
            ctx,
            jobs: jobs.into(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Bunshin for Sequential {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            jobs: stream::iter(self.jobs.clone().unwrap().into_iter())
                .then(|j| async move { j.bunshin().await })
                .collect::<Vec<Job>>()
                .await
                .into(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl Checker for Sequential {
    async fn is_finished(&self) -> JfResult<bool> {
        match self.handle.lock().await.deref() {
            Some(handle) => Ok(handle.is_finished()),
            None => Ok(false), // not started yet
        }
    }
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn start(&self) -> JfResult<Self> {
        let mut logger = self.ctx.logger();
        logger.debug("Sequential starting...").await?;
        let handle: JfHandle = tokio::spawn({
            let mut jobs = self.jobs.clone().unwrap();
            let is_cancelled = self.is_cancelled.clone();
            let job = jobs[0].link_cancel(is_cancelled.clone()).start().await?; // start first job immediately

            async move {
                job.join().await?;
                for mut job in jobs.into_iter().skip(1) {
                    //TODO: remove this statement
                    if is_cancelled.load(Ordering::Relaxed) {
                        job.cancel().await?;
                        continue;
                    }
                    job.link_cancel(is_cancelled.clone())
                        .start()
                        .await?
                        .join()
                        .await?;
                }
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        logger.debug("Sequential started").await?;
        Ok(self.clone())
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

impl From<Sequential> for Job {
    fn from(value: Sequential) -> Self {
        Self::Sequential(value)
    }
}
