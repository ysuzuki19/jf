mod args;
mod completion_script;
mod job_controller;
mod models;

use crate::{
    ctx::{logger, Ctx},
    error::JfResult,
};

pub use self::args::Args;
use self::models::{
    action::{Action, CliAction},
    Opts,
};

pub struct Cli<LR: logger::LogWriter> {
    ctx: Ctx<LR>,
    action: Action,
    opts: Opts,
}

impl<LR: logger::LogWriter> Cli<LR> {
    pub fn load(args: Args) -> JfResult<Self> {
        let (ctx, action, opts) = args.setup()?;
        Ok(Self { ctx, action, opts })
    }

    pub fn ctx(&self) -> &Ctx<LR> {
        &self.ctx
    }

    pub async fn run(self) -> JfResult<()> {
        self.action.run(self.ctx, self.opts).await
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{args::fixtures, models::action::Configured};
    use crate::testutil::*;

    use super::*;

    impl Fixture for Cli<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Self {
                ctx: Fixture::fixture(),
                action: Fixture::fixture(),
                opts: Fixture::fixture(),
            }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn load() -> JfResult<()> {
        let args = Args::parse_from(args::fixtures::SIMPLE);
        let cli = Cli::<MockLogWriter>::load(args)?;
        assert!(cli.ctx() == &Ctx::new(logger::LogLevel::Info));
        assert!(cli.action == Configured::Run(fixtures::JOB_NAME.into()).into());
        assert!(cli.opts == Opts::default());
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn run() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let cli = Cli::fixture();
                assert!(cli.ctx() == &Ctx::fixture());
                assert!(cli.action == Action::fixture());
                cli.run().await?;
                Ok(())
            },
        )
    }
}
