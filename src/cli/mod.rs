mod args;
mod completion_script;
mod job_controller;
mod models;

use clap::Parser;

use crate::{cfg, error::JfResult};

pub use self::args::Args;
use self::models::{
    action::{Action, Configured, Static},
    Ctx, Opts,
};
pub use models::LogLevel;

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
        match self.action {
            Action::Configured(act) => {
                let cfg = cfg::Cfg::load(self.opts.cfg)?;
                let jc = job_controller::JobController::new(cfg)?;
                match act {
                    Configured::List => self.ctx.logger.log(jc.list().join(" ")),
                    Configured::Validate => jc.validate()?,
                    Configured::Run(job_name) => jc.run(job_name).await?,
                    Configured::Description(job_name) => {
                        self.ctx.logger.log(jc.description(job_name)?)
                    }
                }
            }
            Action::Static(act) => match act {
                Static::Help => <Args as clap::CommandFactory>::command().print_help()?,
                Static::Completion(shell) => {
                    self.ctx.logger.log(completion_script::generate(shell))
                }
            },
        }
        Ok(())
    }
}
