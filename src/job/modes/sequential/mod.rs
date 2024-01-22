#[cfg(test)]
mod test;

use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{
    error::{InternalError, JfResult},
    job::{types::JfHandle, Job, Runner},
    jobdef::{Agent, JobdefPool},
};

#[derive(Clone, serde::Deserialize)]
pub struct SequentialParams {
    pub jobs: Vec<String>,
}

#[derive(Clone)]
pub struct Sequential {
    jobs: Vec<Job>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JfHandle>>>,
}

impl Sequential {
    pub fn new(params: SequentialParams, pool: JobdefPool) -> JfResult<Self> {
        if params.jobs.is_empty() {
            return Err(InternalError::MustHaveAtLeastOneJob("sequential".into()).into());
        }
        let jobs = params
            .jobs
            .into_iter()
            .map(|job_name| pool.build(job_name, Agent::Job))
            .collect::<JfResult<Vec<Job>>>()?;
        Ok(Self {
            jobs,
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Sequential {
    async fn start(&self) -> JfResult<Self> {
        let handle: JfHandle = tokio::spawn({
            let job = self.jobs[0].start().await?; // start first job immediately
            let jobs = self.jobs.clone();
            let is_cancelled = self.is_cancelled.clone();

            async move {
                job.wait_with_cancel(is_cancelled.clone()).await?;
                for job in jobs.iter().skip(1) {
                    if is_cancelled.load(Ordering::Relaxed) {
                        job.cancel().await?;
                        continue;
                    }
                    job.start().await?;
                    job.wait_with_cancel(is_cancelled.clone()).await?;
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

    fn bunshin(&self) -> Self {
        Self {
            jobs: self.jobs.iter().map(|job| job.bunshin()).collect(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Sequential> for Job {
    fn from(value: Sequential) -> Self {
        Self::Sequential(value)
    }
}
