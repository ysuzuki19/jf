// SPDX-License-Identifier: MPL-2.0
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
            Configured::Validate => match jc.validate(ctx.clone()) {
                Ok(_) => ctx.logger().force("All jobs are valid").await?,
                Err(e) => ctx.logger().force(format!("{}", e)).await?,
            },
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
    use crate::util::testutil::*;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() {
        println!("{:?}", Configured::List);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn list() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                Configured::List
                    .run(Ctx::async_fixture().await, Fixture::fixture())
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
                Configured::Validate
                    .run(Ctx::async_fixture().await, Fixture::fixture())
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
                Configured::Run(fixtures::JOB_NAME.to_owned())
                    .run(Ctx::async_fixture().await, Fixture::fixture())
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
                Configured::Description(fixtures::JOB_NAME.to_owned())
                    .run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
                Ok(())
            },
        )
    }
}
