mod args;
mod completion_script;
mod job_controller;
mod models;

use crate::error::JfResult;

pub use self::args::Args;
use self::models::{
    action::{Action, CliAction},
    Ctx, Opts,
};

pub struct Cli {
    ctx: Ctx,
    action: Action,
    opts: Opts,
}

impl Cli {
    pub fn load(args: Args) -> JfResult<Self> {
        let (ctx, action, opts) = args.setup()?;
        Ok(Self { ctx, action, opts })
    }

    pub fn ctx(&self) -> &Ctx {
        &self.ctx
    }

    pub async fn run(self) -> JfResult<()> {
        self.action.run(self.ctx, self.opts).await
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{
        args::fixtures,
        models::action::{Configured, Statics},
    };

    use super::*;

    #[test]
    fn load() -> JfResult<()> {
        let args = Args::parse_from(args::fixtures::SIMPLE);
        let cli = Cli::load(args)?;
        assert_eq!(cli.ctx(), &Ctx::default());
        assert_eq!(
            cli.action,
            Configured::Run(fixtures::JOB_NAME.into()).into()
        );
        assert_eq!(cli.opts, Opts::default());
        Ok(())
    }

    #[tokio::test]
    async fn run() -> JfResult<()> {
        let cli = Cli {
            ctx: Ctx::fixture(),
            action: Action::Statics(Statics::Help),
            opts: Opts::fixture(),
        };
        assert_eq!(cli.ctx(), &Ctx::fixture());
        cli.run().await?;
        Ok(())
    }
}
