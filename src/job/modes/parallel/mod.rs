// SPDX-License-Identifier: MPL-2.0
#[cfg(test)]
mod tests;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    ctx::Ctx,
    job::{
        canceller::Canceller, finish_notify::FinishNotify, join_status::JoinStatus, runner::*, Job,
    },
    jobdef::{Agent, JobdefPool},
    util::error::JfResult,
};

#[derive(Clone, serde::Deserialize)]
pub struct ParallelParams {
    pub jobs: Vec<String>,
}

#[derive(Clone)]
pub struct Parallel {
    ctx: Ctx,
    jobs: Vec<Job>,
    canceller: Canceller,
    running_jobs: Arc<Mutex<Vec<Job>>>,
    finish_handle: Arc<Mutex<Option<JoinHandle<JfResult<()>>>>>,
    finish_notify: Arc<FinishNotify>,
}

impl Parallel {
    pub fn new(ctx: Ctx, params: ParallelParams, pool: JobdefPool) -> JfResult<Self> {
        let jobs = params
            .jobs
            .into_iter()
            .map(|job_name| pool.build(ctx.clone(), job_name, Agent::Job))
            .collect::<JfResult<Vec<Job>>>()?;
        Ok(Self {
            ctx,
            jobs: jobs.clone(),
            canceller: Canceller::new(),
            running_jobs: Arc::new(Mutex::new(jobs)),
            finish_handle: Arc::new(Mutex::new(None)),
            finish_notify: FinishNotify::new_arc(),
        })
    }
}

#[async_trait::async_trait]
impl Bunshin for Parallel {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            jobs: self.jobs.bunshin().await,
            canceller: Canceller::new(),
            running_jobs: Arc::new(Mutex::new(self.running_jobs.lock().await.bunshin().await)),
            finish_handle: Arc::new(Mutex::new(None)),
            finish_notify: FinishNotify::new_arc(),
        }
    }
}

#[async_trait::async_trait]
impl Checker for Parallel {
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
impl Runner for Parallel {
    async fn start(&self) -> JfResult<Self> {
        let mut logger = self.ctx.logger();
        logger.debug("Parallel starting...").await?;
        for job in self.running_jobs.lock().await.deref() {
            job.start().await?;
        }
        let handle = tokio::spawn({
            let jobs = self.jobs.clone();
            let finish_notify = self.finish_notify.clone();
            async move {
                for job in jobs {
                    job.join().await?;
                }
                finish_notify.notify();
                Ok(())
            }
        });
        self.finish_handle.lock().await.replace(handle);
        logger.debug("Parallel started").await?;
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        for job in self.running_jobs.lock().await.deref() {
            job.cancel().await?;
        }
        self.canceller.cancel();
        Ok(self.clone())
    }

    async fn join(&self) -> JfResult<JoinStatus> {
        self.finish_notify.wait().await;
        for job in self.running_jobs.lock().await.deref_mut() {
            if let JoinStatus::Failed = job.join().await? {
                return Ok(JoinStatus::Failed);
            }
        }
        return Ok(JoinStatus::Succeed);
    }

    fn set_canceller(&mut self, canceller: Canceller) -> Self {
        self.canceller = canceller;
        self.clone()
    }
}

impl From<Parallel> for Job {
    fn from(value: Parallel) -> Self {
        Self::Parallel(value)
    }
}
