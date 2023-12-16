mod agent;
mod pool;

pub use self::agent::Agent;
pub use self::pool::JobdefPool;
use crate::{
    cfg::JobCfg,
    error::{JfError, JfResult},
    job::Job,
};

pub struct Jobdef {
    name: String,
    private: bool,
    description: String,
    job_cfg: JobCfg,
}

impl Jobdef {
    pub fn new(name: String, job_cfg: JobCfg) -> JfResult<Self> {
        Ok(Self {
            name,
            private: job_cfg.private(),
            description: job_cfg.description(),
            job_cfg,
        })
    }

    fn visibility_guard(&self, agent: Agent) -> JfResult<()> {
        if !self.private {
            return Ok(());
        }
        match agent {
            Agent::Cli => Err(JfError::Custom(format!(
                "job.{} is private\nPlease remove `private = true` if you run",
                self.name
            ))),
            _ => Ok(()),
        }
    }

    fn build(&self, pool: JobdefPool, agent: Agent) -> JfResult<Job> {
        self.visibility_guard(agent)?;
        Job::new(self.job_cfg.clone(), pool)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl TryFrom<(String, JobCfg)> for Jobdef {
    type Error = JfError;

    fn try_from(value: (String, JobCfg)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}