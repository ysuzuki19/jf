#[cfg(test)]
mod test;

use std::{
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::{
    error::JfResult,
    job::{runner::Runner, types::JfHandle, Job},
    jobdef::{Agent, JobdefPool},
};

#[derive(Clone, serde::Deserialize)]
pub struct ParallelParams {
    pub jobs: Vec<String>,
}

#[derive(Clone)]
pub struct Parallel {
    jobs: Vec<Job>,
    handles: Arc<Mutex<Option<Vec<JfHandle>>>>,
    is_cancelled: Arc<AtomicBool>,
}

impl Parallel {
    pub fn new(params: ParallelParams, pool: JobdefPool) -> JfResult<Self> {
        let jobs = params
            .jobs
            .into_iter()
            .map(|job_name| pool.build(job_name, Agent::Job))
            .collect::<JfResult<Vec<Job>>>()?;
        Ok(Self {
            jobs,
            handles: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Parallel {
    async fn start(&self) -> JfResult<Self> {
        let mut handles = Vec::new();
        for job in self.jobs.clone() {
            let handle: JfHandle = tokio::spawn({
                let is_cancelled = self.is_cancelled.clone();
                job.start().await?;
                async move {
                    job.wait_with_cancel(is_cancelled).await?;
                    Ok(())
                }
            });
            handles.push(handle);
        }
        self.handles.lock().await.replace(handles);

        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        match self.handles.lock().await.deref_mut() {
            Some(hs) => Ok(hs.iter().all(|h| h.is_finished())),
            None => Ok(false), // not started yet
        }
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.is_cancelled.store(true, Ordering::SeqCst);
        if let Some(handles) = self.handles.lock().await.deref_mut() {
            for handle in handles {
                let _ = handle.await?;
            }
        }
        Ok(self.clone())
    }

    fn bunshin(&self) -> Self {
        Self {
            jobs: self.jobs.iter().map(|job| job.bunshin()).collect(),
            handles: Arc::new(Mutex::new(None)),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl From<Parallel> for Job {
    fn from(value: Parallel) -> Self {
        Self::Parallel(value)
    }
}