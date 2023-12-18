use crate::{
    cfg::Cfg,
    error::JfResult,
    job::Runner,
    jobdef::{Agent, JobdefPool},
};

pub struct Commander {
    pool: JobdefPool,
}

impl Commander {
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

    pub async fn run(&self, job_name: String) -> JfResult<()> {
        self.pool
            .build(job_name, Agent::Cli)?
            .start()
            .await?
            .wait()
            .await
    }

    pub fn description(&self, job_name: String) -> JfResult<&String> {
        self.pool.description(job_name)
    }

    pub fn list(&self) -> Vec<String> {
        let mut job_names = self.pool.list();
        job_names.sort();
        job_names
    }
}
