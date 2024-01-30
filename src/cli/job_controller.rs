use crate::{
    cfg::Cfg,
    ctx::{logger, Ctx},
    error::JfResult,
    job::Runner,
    jobdef::{Agent, JobdefPool},
};

pub struct JobController {
    pool: JobdefPool,
}

impl JobController {
    pub fn new(cfg: Cfg) -> JfResult<Self> {
        let job_vec = cfg
            .jobs
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<JfResult<_>>()?;
        Ok(Self {
            pool: JobdefPool::new(job_vec),
        })
    }

    pub async fn run<LR: logger::LogWriter>(&self, ctx: Ctx<LR>, job_name: String) -> JfResult<()> {
        self.pool
            .build::<LR>(job_name, Agent::Cli)?
            .start(ctx)
            .await?
            .join()
            .await?;
        Ok(())
    }

    pub fn description(&self, job_name: String) -> JfResult<&String> {
        self.pool.description(job_name)
    }

    pub fn list_public(&self) -> Vec<String> {
        let mut job_names = self.pool.list_public();
        job_names.sort();
        job_names
    }

    pub fn validate(&self) -> JfResult<()> {
        self.pool.validate()
    }
}
