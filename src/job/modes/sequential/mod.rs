// SPDX-License-Identifier: MPL-2.0
#[cfg(test)]
mod tests;

use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    ctx::Ctx,
    job::{
        canceller::Canceller, finish_notify::FinishNotify, join_status::JoinStatus, runner::*, Job,
    },
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
    finish_notify: Arc<FinishNotify>,
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
            finish_notify: FinishNotify::new_arc(),
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
            finish_notify: FinishNotify::new_arc(),
        }
    }
}

#[async_trait::async_trait]
impl Checker for Sequential {
    async fn is_finished(&self) -> JfResult<bool> {
        Ok(self.finish_notify.is_finished())
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
            let finish_notify = self.finish_notify.clone();

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
                        finish_notify.notify();
                        return Ok(JoinStatus::Failed);
                    }
                }
                finish_notify.notify();
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
        self.finish_notify.wait().await;
        match self.handle.lock().await.deref_mut() {
            Some(handle) => handle.await?,
            None => Ok(JoinStatus::Succeed), // not started yet
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
