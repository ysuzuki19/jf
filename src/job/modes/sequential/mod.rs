// SPDX-License-Identifier: MPL-2.0
#[cfg(test)]
mod tests;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use tokio::sync::Mutex;

use crate::{
    ctx::Ctx,
    job::{canceller::Canceller, join_status::JoinStatus, runner::*, Job},
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
    canceller: Canceller,
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
            canceller: Canceller::new(),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Bunshin for Sequential {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            jobs: self.jobs.clone().into_inner().bunshin().await.into(),
            canceller: Canceller::new(),
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
            let mut jobs = self.jobs.clone().into_inner();
            let canceller = self.canceller.clone();
            let job = jobs[0].set_canceller(canceller.clone()).start().await?; // start first job immediately

            async move {
                job.join().await?;
                for mut job in jobs.into_iter().skip(1) {
                    //TODO: remove this statement
                    if canceller.is_canceled() {
                        job.cancel().await?;
                        break;
                    }
                    let status = job
                        .set_canceller(canceller.clone())
                        .start()
                        .await?
                        .join()
                        .await?;
                    if status.is_failed() {
                        return Ok(JoinStatus::Failed);
                    }
                }
                Ok(JoinStatus::Succeed)
            }
        });
        self.handle.lock().await.replace(handle);
        logger.debug("Sequential started").await?;
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.canceller.cancel();
        Ok(self.clone())
    }

    async fn join(&self) -> JfResult<JoinStatus> {
        loop {
            if self.is_finished().await? {
                return match self.handle.lock().await.deref_mut() {
                    Some(handle) => {
                        return handle.await?;
                    }
                    None => Ok(JoinStatus::Succeed), // not started yet
                };
            }

            interval().await;
        }
    }

    fn set_canceller(&mut self, canceller: Canceller) -> Self {
        self.canceller = canceller;
        self.clone()
    }
}

impl From<Sequential> for Job {
    fn from(value: Sequential) -> Self {
        Self::Sequential(value)
    }
}
