mod args;
mod completion_script;
mod job_controller;
mod models;

use clap::Parser;

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
    pub fn load() -> JfResult<Self> {
        let (ctx, action, opts) = Args::parse().setup()?;
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
    use crate::cli::models::action::Statics;

    use super::*;

    #[tokio::test]
    async fn load() -> JfResult<()> {
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
