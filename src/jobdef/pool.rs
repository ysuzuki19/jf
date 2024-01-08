use std::{collections::HashMap, sync::Arc};

use super::{Agent, Jobdef};
use crate::{
    error::{InternalError, JfError, JfResult},
    job::Job,
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
                if jobdef.visibility().is_public() {
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
            .map(|jobdef| jobdef.build(self.clone(), Agent::Job))
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
            .ok_or(InternalError::JobdefNotFound(job_name).into())
    }

    pub fn build(&self, job_name: String, agent: Agent) -> JfResult<Job> {
        self.get(job_name)?.build(self.clone(), agent)
    }

    pub fn description(&self, job_name: String) -> JfResult<&String> {
        Ok(self.get(job_name)?.description())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        cfg::job_cfg::{CommonCfg, JobCfg, MockCfg, Visibility, WatchCfg},
        testutil::{Fixture, TryFixture},
    };

    impl TryFixture for JobdefPool {
        #[cfg_attr(coverage, coverage(off))]
        fn try_gen() -> JfResult<Self> {
            let jobdef = TryFixture::try_gen()?;
            Ok(Self::new(vec![jobdef]))
        }
    }

    #[test]
    fn test() -> JfResult<()> {
        let pool = JobdefPool::new(vec![
            Jobdef::new(
                "job1".into(),
                JobCfg::Mock(MockCfg {
                    common: CommonCfg::new(Visibility::Public, "job1-desc".into()),
                    params: Fixture::gen(),
                }),
            )?,
            Jobdef::new(
                "job2".into(),
                JobCfg::Mock(MockCfg {
                    common: CommonCfg::new(Visibility::Public, "job2-desc".into()),
                    params: Fixture::gen(),
                }),
            )?,
            Jobdef::new(
                "job3".into(),
                JobCfg::Mock(MockCfg {
                    common: CommonCfg::new(Visibility::Private, "job3-desc".into()),
                    params: Fixture::gen(),
                }),
            )?,
        ]);
        assert_eq!(pool.list_public().len(), 2);
        assert!(pool.validate().is_ok());
        assert!(pool.build("job1".into(), Agent::Job).is_ok());
        assert!(pool.build("job1".into(), Agent::Cli).is_ok());
        assert!(pool.build("job3".into(), Agent::Job).is_ok());
        assert!(pool.build("job3".into(), Agent::Cli).is_err());
        assert_eq!(pool.description("job1".into())?, "job1-desc");
        Ok(())
    }

    #[test]
    fn fail() -> JfResult<()> {
        let pool = JobdefPool::new(vec![
            Jobdef::new("job1".into(), JobCfg::Mock(Fixture::gen()))?,
            Jobdef::new("job2".into(), JobCfg::Mock(Fixture::gen()))?,
            Jobdef::new(
                "job3".into(),
                JobCfg::Watch(WatchCfg {
                    common: CommonCfg::new(Visibility::Private, "job3-desc".into()),
                    params: Fixture::gen(),
                }),
            )?,
        ]);
        assert_eq!(pool.list_public().len(), 2);
        assert!(pool.validate().is_err());
        assert!(pool.build("job1".into(), Agent::Job).is_ok());
        assert!(pool.build("job1".into(), Agent::Cli).is_ok());
        assert!(pool.build("job3".into(), Agent::Job).is_err());
        assert!(pool.build("job3".into(), Agent::Cli).is_err());
        assert_eq!(pool.description("job1".into())?, "");
        assert_eq!(pool.description("job3".into())?, "job3-desc");
        Ok(())
    }
}
