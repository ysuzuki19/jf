use crate::{
    cfg,
    cli::{
        job_controller,
        models::{Ctx, Opts},
    },
    error::JfResult,
};

use super::{Action, CliAction};

impl From<Configured> for Action {
    fn from(c: Configured) -> Self {
        Action::Configured(c)
    }
}

// Action with job configuration
pub enum Configured {
    List,
    Validate,
    Description(String),
    Run(String),
}

#[async_trait::async_trait]
impl CliAction for Configured {
    async fn run(self, ctx: Ctx, opts: Opts) -> JfResult<()> {
        let cfg = cfg::Cfg::load(opts.cfg)?;
        let jc = job_controller::JobController::new(cfg)?;
        match self {
            Configured::List => ctx.logger.log(jc.list().join(" ")),
            Configured::Validate => jc.validate()?,
            Configured::Run(job_name) => jc.run(job_name).await?,
            Configured::Description(job_name) => ctx.logger.log(jc.description(job_name)?),
        }
        Ok(())
    }
}
