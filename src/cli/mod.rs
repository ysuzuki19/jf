mod args;
mod completion_script;
mod job_controller;
mod models;

use crate::{ctx::Ctx, log_worker::Logger, util::error::JfResult};

pub use self::args::Args;
use self::models::{
    action::{Action, CliAction},
    Opts,
};

pub struct Cli {
    ctx: Ctx,
    action: Action,
    opts: Opts,
}

impl Cli {
    pub fn load(logger: Logger, args: Args) -> JfResult<Self> {
        let (ctx, action, opts) = args.setup(logger)?;
        Ok(Self { ctx, action, opts })
    }

    pub async fn run(self) -> JfResult<()> {
        self.action.run(self.ctx, self.opts).await
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{args::fixtures, models::action::Configured};
    use crate::log_worker::LogWorkerMock;
    use crate::util::testutil::*;

    use super::*;

    impl AsyncFixture for Cli {
        #[cfg_attr(coverage, coverage(off))]
        async fn async_fixture() -> Self {
            Self {
                ctx: Ctx::async_fixture().await,
                action: Fixture::fixture(),
                opts: Fixture::fixture(),
            }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn load() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async move {
                let args = Args::parse_from(args::fixtures::SIMPLE);
                let log_worker_mock = LogWorkerMock::new().await;
                let cli = Cli::load(log_worker_mock.logger.clone(), args)?;
                assert_eq!(cli.ctx, Ctx::new(log_worker_mock.logger));
                assert_eq!(
                    cli.action,
                    Configured::Run(fixtures::JOB_NAME.into()).into()
                );
                assert_eq!(cli.opts, Opts::default());
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
                let cli = Cli::async_fixture().await;
                assert_eq!(cli.ctx, Ctx::async_fixture().await);
                assert_eq!(cli.action, Action::fixture());
                cli.run().await?;
                Ok(())
            },
        )
    }
}
