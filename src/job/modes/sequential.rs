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

#[derive(Debug, Clone, serde::Deserialize)]
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
        if let Some(handle) = self.handle.lock().await.deref() {
            Ok(handle.is_finished())
        } else {
            Ok(false) // not yet started
        }
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
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
        Job::Sequential(value)
    }
}

#[cfg(test)]
mod fixtures {
    use crate::jobdef::Jobdef;

    use super::*;

    pub const CFG_CONTENT: &str = r#"
mode = "mock"
each_sleep_time = 100
sleep_count = 3
"#;

    pub fn pool() -> JfResult<JobdefPool> {
        let cfg = toml::from_str(CFG_CONTENT)?;
        let jobdefs = vec![Jobdef::new("fast".into(), cfg)?];
        Ok(JobdefPool::new(jobdefs))
    }
}

#[cfg(test)]
mod test {
    use crate::{job::runner, testutil::Fixture};

    use super::*;

    impl Fixture for SequentialParams {
        fn fixture() -> Self {
            Self {
                jobs: vec!["fast".into(), "fast".into()],
            }
        }
    }

    #[test]
    fn invalid_new_with_empty_job() -> JfResult<()> {
        let params = SequentialParams { jobs: vec![] };
        let pool = fixtures::pool()?;
        assert!(Sequential::new(params, pool).is_err());
        Ok(())
    }

    #[test]
    fn invalid_new_with_unknown_job() -> JfResult<()> {
        let params = SequentialParams {
            jobs: vec!["unknown".into()],
        };
        let pool = fixtures::pool()?;
        let must_fail = Sequential::new(params, pool);
        assert!(must_fail.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn new() -> JfResult<()> {
        let pool = fixtures::pool()?;
        Sequential::new(Fixture::fixture(), pool)?;
        Ok(())
    }

    #[tokio::test]
    async fn start() -> JfResult<()> {
        let pool = fixtures::pool()?;
        let s = Sequential::new(Fixture::fixture(), pool)?.start().await?;
        assert!(!s.is_finished().await?);
        for (index, job) in s.jobs.iter().enumerate() {
            if index == 0 {
                // first job is started immediately
                job.as_mock().assert_is_started_eq(true);
            } else {
                // other jobs are not started yet
                job.as_mock().assert_is_started_eq(false);
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn cancel() -> JfResult<()> {
        let pool = fixtures::pool()?;
        let s = Sequential::new(Fixture::fixture(), pool)?.start().await?;
        s.cancel().await?;
        runner::sleep().await; // sleep for job interval
        assert!(s.is_cancelled.load(Ordering::Relaxed));
        Ok(())
    }

    #[tokio::test]
    async fn wait() -> JfResult<()> {
        let pool = fixtures::pool()?;
        let s = Sequential::new(Fixture::fixture(), pool)?.start().await?;
        s.wait().await?;
        s.is_finished().await?;
        for job in s.jobs.iter() {
            job.as_mock().assert_is_finished_eq(true);
        }
        Ok(())
    }

    #[tokio::test]
    async fn bunshin() -> JfResult<()> {
        let pool = fixtures::pool()?;
        let origin = Sequential::new(Fixture::fixture(), pool)?;
        origin.start().await?.cancel().await?;

        let bunshin = origin.bunshin();
        assert_eq!(origin.jobs.len(), bunshin.jobs.len());
        for (bunshin_job, origin_job) in bunshin.jobs.iter().zip(origin.jobs) {
            bunshin_job
                .as_mock()
                .assert_id_ne(origin_job.as_mock().id())
                .assert_is_started_eq(false)
                .assert_is_cancelled_eq(false);
        }
        Ok(())
    }

    #[tokio::test]
    async fn is_finished_not_yet_started() -> JfResult<()> {
        let pool = fixtures::pool()?;
        let s = Sequential::new(Fixture::fixture(), pool)?;
        assert!(!s.is_finished().await?);
        Ok(())
    }
}
