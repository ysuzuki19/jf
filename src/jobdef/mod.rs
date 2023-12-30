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
        Job::new(self.job_cfg.clone(), pool)
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
