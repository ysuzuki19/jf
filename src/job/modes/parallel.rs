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

#[derive(Debug, Clone, serde::Deserialize)]
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
            handles: Arc::new(Mutex::new(Some(Vec::new()))),
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
        if let Some(handles) = self.clone().handles.lock().await.deref_mut() {
            let all_finished = handles.iter().all(|h| h.is_finished());
            if !all_finished {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::SeqCst);
        if let Some(handles) = self.handles.lock().await.deref_mut() {
            for handle in handles {
                let _ = handle.await?;
            }
        }
        Ok(())
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
        Job::Parallel(value)
    }
}

#[cfg(test)]
mod fixtures {
    use crate::{cfg::job_cfg::JobCfg, jobdef::Jobdef};

    use super::*;

    pub const CFG_CONTENT: &str = r#"
mode = "mock"
each_sleep_time = 100
sleep_count = 3
"#;

    pub fn params() -> super::ParallelParams {
        super::ParallelParams {
            jobs: vec!["fast".into(), "fast".into()],
        }
    }

    pub fn pool() -> JfResult<super::JobdefPool> {
        let mock_cfg: JobCfg = toml::from_str(CFG_CONTENT)?;
        let jobdefs = vec![Jobdef::new("fast".into(), mock_cfg).unwrap()];
        Ok(super::JobdefPool::new(jobdefs))
    }
}

#[cfg(test)]
mod test {
    use crate::{error::JfResult, job::Runner, jobdef::JobdefPool};

    use super::*;

    #[tokio::test]
    async fn invalid_new_with_unknown_job() -> JfResult<()> {
        let must_fail = Parallel::new(
            ParallelParams {
                jobs: vec!["mock".into(), "mock".into()],
            },
            JobdefPool::new(vec![]),
        );
        assert!(must_fail.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn new() -> JfResult<()> {
        let params = fixtures::params();
        let pool = fixtures::pool()?;
        let p = Parallel::new(params, pool)?;
        assert!(p.jobs.len() == 2);
        Ok(())
    }

    #[tokio::test]
    async fn start() -> JfResult<()> {
        let params = fixtures::params();
        let pool = fixtures::pool()?;
        let p = Parallel::new(params, pool)?;
        p.start().await?;
        for job in p.jobs {
            job.as_mock().assert_is_started_eq(true);
        }
        Ok(())
    }

    #[tokio::test]
    async fn cancel() -> JfResult<()> {
        let params = fixtures::params();
        let pool = fixtures::pool()?;
        let p = Parallel::new(params, pool)?;
        p.start().await?.cancel().await?;
        for job in p.jobs {
            job.as_mock()
                .assert_is_started_eq(true)
                .assert_is_cancelled_eq(true);
        }
        Ok(())
    }

    #[tokio::test]
    async fn wait() -> JfResult<()> {
        let params = fixtures::params();
        let pool = fixtures::pool()?;
        let p = Parallel::new(params, pool)?;
        p.start().await?.wait().await?;
        for job in p.jobs {
            job.as_mock()
                .assert_is_started_eq(true)
                .assert_is_finished_eq(true);
        }
        Ok(())
    }

    #[tokio::test]
    async fn bunshin() -> JfResult<()> {
        let params = fixtures::params();
        let pool = fixtures::pool()?;
        let origin = Parallel::new(params, pool)?;
        origin.start().await?.cancel().await?;

        let bunshin = origin.bunshin();
        assert_eq!(origin.jobs.len(), bunshin.jobs.len());
        for (bunshin_job, origin_job) in bunshin.jobs.iter().zip(origin.jobs) {
            bunshin_job
                .as_mock()
                .assert_id_ne(origin_job.as_mock().id())
                .assert_is_started_eq(false);
        }
        Ok(())
    }
}
