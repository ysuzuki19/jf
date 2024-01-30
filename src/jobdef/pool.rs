use std::{collections::HashMap, sync::Arc};

use super::{Agent, Jobdef};
use crate::{
    ctx::logger::{JfStdout, LogWriter},
    job::Job,
    util::error::{IntoJfError, JfError, JfResult},
};

#[derive(Clone)]
pub struct JobdefPool {
    map: Arc<HashMap<String, Jobdef>>,
}

impl JobdefPool {
    pub fn new(jobdefs: Vec<Jobdef>) -> Self {
        let map = HashMap::from_iter(jobdefs.into_iter().map(|jd| (jd.name().to_owned(), jd)));
        Self { map: Arc::new(map) }
    }

    pub fn list_public(&self) -> Vec<String> {
        self.map
            .values()
            .filter_map(|jobdef| {
                if jobdef.is_public() {
                    Some(jobdef.name().to_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn validate(&self) -> JfResult<()> {
        let errs = self
            .map
            .values()
            .map(|jobdef| jobdef.build::<JfStdout>(self.clone(), Agent::Job))
            .filter_map(|res| match res {
                Ok(_) => None,
                Err(e) => Some(e),
            })
            .collect::<Vec<_>>();
        if errs.is_empty() {
            Ok(())
        } else {
            Err(JfError::Multi(errs))
        }
    }

    fn get(&self, job_name: String) -> JfResult<&Jobdef> {
        self.map
            .get(&job_name)
            .ok_or(format!("Jobdef(name={}) not found", job_name).into_jf_error())
    }

    pub fn build<LR: LogWriter>(&self, job_name: String, agent: Agent) -> JfResult<Job<LR>> {
        self.get(job_name)?.build(self.clone(), agent)
    }

    pub fn description(&self, job_name: String) -> JfResult<&String> {
        Ok(self.get(job_name)?.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cfg::job_cfg::{CommonCfg, JobCfg, MockCfg, Visibility, WatchCfg},
        util::testutil::*,
    };

    impl TryFixture for JobdefPool {
        #[cfg_attr(coverage, coverage(off))]
        fn try_fixture() -> JfResult<Self> {
            let jobdef = TryFixture::try_fixture()?;
            Ok(Self::new(vec![jobdef]))
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test() -> JfResult<()> {
        let pool = JobdefPool::new(vec![
            Jobdef::new(
                "job1".into(),
                JobCfg::Mock(MockCfg {
                    common: CommonCfg::new(Visibility::Public, "job1-desc".into()),
                    params: Fixture::fixture(),
                }),
            )?,
            Jobdef::new(
                "job2".into(),
                JobCfg::Mock(MockCfg {
                    common: CommonCfg::new(Visibility::Public, "job2-desc".into()),
                    params: Fixture::fixture(),
                }),
            )?,
            Jobdef::new(
                "job3".into(),
                JobCfg::Mock(MockCfg {
                    common: CommonCfg::new(Visibility::Private, "job3-desc".into()),
                    params: Fixture::fixture(),
                }),
            )?,
        ]);
        assert_eq!(pool.list_public().len(), 2);
        assert!(pool.validate().is_ok());
        assert!(pool.build::<JfStdout>("job1".into(), Agent::Job).is_ok());
        assert!(pool.build::<JfStdout>("job1".into(), Agent::Cli).is_ok());
        assert!(pool.build::<JfStdout>("job3".into(), Agent::Job).is_ok());
        assert!(pool.build::<JfStdout>("job3".into(), Agent::Cli).is_err());
        assert_eq!(pool.description("job1".into())?, "job1-desc");
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn fail() -> JfResult<()> {
        let pool = JobdefPool::new(vec![
            Jobdef::new("job1".into(), JobCfg::Mock(Fixture::fixture()))?,
            Jobdef::new("job2".into(), JobCfg::Mock(Fixture::fixture()))?,
            Jobdef::new(
                "job3".into(),
                JobCfg::Watch(WatchCfg {
                    common: CommonCfg::new(Visibility::Private, "job3-desc".into()),
                    params: Fixture::fixture(),
                }),
            )?,
        ]);
        assert_eq!(pool.list_public().len(), 2);
        assert!(pool.validate().is_err());
        assert!(pool.build::<JfStdout>("job1".into(), Agent::Job).is_ok());
        assert!(pool.build::<JfStdout>("job1".into(), Agent::Cli).is_ok());
        assert!(pool.build::<JfStdout>("job3".into(), Agent::Job).is_err());
        assert!(pool.build::<JfStdout>("job3".into(), Agent::Cli).is_err());
        assert_eq!(pool.description("job1".into())?, "");
        assert_eq!(pool.description("job3".into())?, "job3-desc");
        Ok(())
    }
}
