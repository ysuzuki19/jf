mod agent;
mod pool;

pub use self::agent::Agent;
pub use self::pool::JobdefPool;
use crate::{
    cfg::job_cfg::{JobCfg, Visibility},
    error::{InternalError, JfError, JfResult},
    job::Job,
};

pub struct Jobdef {
    name: String,
    visibility: Visibility,
    description: String,
    job_cfg: JobCfg,
}

impl Jobdef {
    pub fn new(name: String, job_cfg: JobCfg) -> JfResult<Self> {
        Ok(Self {
            name,
            visibility: job_cfg.visibility().clone(),
            description: job_cfg.description(),
            job_cfg,
        })
    }

    fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    fn visibility_guard(&self, agent: Agent) -> JfResult<()> {
        if self.visibility.is_public() {
            return Ok(());
        }
        match agent {
            Agent::Cli => Err(InternalError::UnexpectedVisibilityPrivate(self.name.clone()).into()),
            _ => Ok(()),
        }
    }

    fn build(&self, pool: JobdefPool, agent: Agent) -> JfResult<Job> {
        self.visibility_guard(agent)?;
        Job::new(&self.job_cfg, pool)
    }

    fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

impl TryFrom<(String, JobCfg)> for Jobdef {
    type Error = JfError;

    fn try_from(value: (String, JobCfg)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cfg::job_cfg::{CommonCfg, MockCfg},
        testutil::{async_test, Fixture, TryFixture},
    };

    use super::*;

    impl TryFixture for Jobdef {
        #[cfg_attr(coverage, coverage(off))]
        fn try_fixture() -> JfResult<Self> {
            Self::new("fast".into(), TryFixture::try_fixture()?)
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn visibility_guard() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let jobdef_public = Jobdef::new(
                    "dummy".into(),
                    JobCfg::Mock(MockCfg {
                        common: CommonCfg::new(Visibility::Public, "".into()),
                        params: Fixture::fixture(),
                    }),
                )?;
                assert!(jobdef_public.visibility_guard(Agent::Job).is_ok());
                assert!(jobdef_public.visibility_guard(Agent::Cli).is_ok());

                let jobdef_private = Jobdef::new(
                    "dummy".into(),
                    JobCfg::Mock(MockCfg {
                        common: CommonCfg::new(Visibility::Private, "".into()),
                        params: Fixture::fixture(),
                    }),
                )?;
                assert!(jobdef_private.visibility_guard(Agent::Job).is_ok());
                assert!(jobdef_private.visibility_guard(Agent::Cli).is_err());
                Ok(())
            },
        )
    }
}
