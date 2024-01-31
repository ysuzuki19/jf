#[cfg(test)]
mod tests;

use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    job::{runner::*, Job},
    jobdef::{Agent, JobdefPool},
    util::error::JfResult,
};

#[derive(Clone, serde::Deserialize)]
pub struct ParallelParams {
    pub jobs: Vec<String>,
}

#[derive(Clone)]
pub struct Parallel<LR: LogWriter> {
    ctx: Ctx<LR>,
    jobs: Vec<Job<LR>>,
    is_cancelled: Arc<AtomicBool>,
    running_jobs: Arc<Mutex<Vec<Job<LR>>>>,
}

impl<LR: LogWriter> Parallel<LR> {
    pub fn new(ctx: Ctx<LR>, params: ParallelParams, pool: JobdefPool) -> JfResult<Self> {
        let jobs = params
            .jobs
            .into_iter()
            .map(|job_name| pool.build(ctx.clone(), job_name, Agent::Job))
            .collect::<JfResult<Vec<Job<LR>>>>()?;
        Ok(Self {
            ctx,
            jobs: jobs.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            running_jobs: Arc::new(Mutex::new(jobs)),
        })
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Parallel<LR> {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            jobs: self.jobs.bunshin().await,
            is_cancelled: Arc::new(AtomicBool::new(false)),
            running_jobs: Arc::new(Mutex::new(self.running_jobs.lock().await.bunshin().await)),
        }
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Checker for Parallel<LR> {
    async fn is_finished(&self) -> JfResult<bool> {
        for job in self.running_jobs.lock().await.deref_mut() {
            if !job.is_finished().await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Runner<LR> for Parallel<LR> {
    async fn start(&self) -> JfResult<Self> {
        let mut logger = self.ctx.logger();
        logger.debug("Parallel starting...").await?;
        for job in self.running_jobs.lock().await.deref_mut() {
            job.link_cancel(self.is_cancelled.clone()).start().await?;
        }
        logger.debug("Parallel started").await?;
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.is_cancelled.store(true, Ordering::SeqCst);
        for job in self.running_jobs.lock().await.deref_mut() {
            job.cancel().await?.join().await?;
        }
        Ok(self.clone())
    }

    fn link_cancel(&mut self, is_cancelled: Arc<AtomicBool>) -> Self {
        self.is_cancelled = is_cancelled;
        self.clone()
    }
}

impl<LR: LogWriter> From<Parallel<LR>> for Job<LR> {
    fn from(value: Parallel<LR>) -> Self {
        Self::Parallel(value)
    }
}
