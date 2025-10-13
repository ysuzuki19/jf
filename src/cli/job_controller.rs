// SPDX-License-Identifier: MPL-2.0
use crate::{
    cfg::Cfg,
    ctx::Ctx,
    job::Runner,
    jobdef::{Agent, JobdefPool},
    util::error::JfResult,
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

    pub async fn run(&self, ctx: Ctx, job_name: String) -> JfResult<()> {
        ctx.engine_log("building job runner");
        let job = self.pool.build(ctx.clone(), job_name, Agent::Cli)?;

        ctx.engine_log("starting job runner");
        job.start().await?.join().await?;
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

    pub fn validate(&self, ctx: Ctx) -> JfResult<()> {
        self.pool.validate(ctx)
    }
}
