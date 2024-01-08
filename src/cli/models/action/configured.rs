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
#[cfg_attr(test, derive(Debug, PartialEq))]
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
            Configured::List => ctx.logger.log(jc.list_public().join(" ")),
            Configured::Validate => jc.validate()?,
            Configured::Run(job_name) => jc.run(job_name).await?,
            Configured::Description(job_name) => ctx.logger.log(jc.description(job_name)?),
        }
        Ok(())
    }
}

#[cfg(test)]
mod fixtures {
    pub const JOB_NAME: &str = "test-fixture";
}

#[cfg(test)]
mod tests {
    use crate::{
        error::JfResult,
        testutil::{async_test, Fixture},
    };

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn list() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let c = Configured::List;
                c.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn validate() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let c = Configured::Validate;
                c.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn run() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let c = Configured::Run(fixtures::JOB_NAME.to_owned());
                c.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn description() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let c = Configured::Description(fixtures::JOB_NAME.to_owned());
                c.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }
}
