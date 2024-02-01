use crate::{
    cfg,
    cli::{job_controller, models::Opts},
    ctx::Ctx,
    util::error::JfResult,
};

use super::{Action, CliAction};

impl From<Configured> for Action {
    fn from(c: Configured) -> Self {
        Action::Configured(c)
    }
}

// Action with job configuration
#[cfg_attr(test, derive(PartialEq, Debug))]
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
            Configured::List => ctx.logger().force(jc.list_public().join(" ")).await?,
            Configured::Validate => jc.validate(ctx)?,
            Configured::Run(name) => jc.run(ctx, name).await?,
            Configured::Description(name) => ctx.logger().force(jc.description(name)?).await?,
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
    use crate::util::{error::JfResult, testutil::*};

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() -> JfResult<()> {
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn list() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let c = Configured::List;
                c.run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
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
                c.run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
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
                c.run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
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
                c.run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
                Ok(())
            },
        )
    }
}
