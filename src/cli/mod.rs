mod action;
mod args;
mod completion_script;
mod containers;
mod job_controller;
mod log_level;

use clap::Parser;

use crate::{cfg, error::JfResult};

pub use self::args::Args;
use self::{
    action::{Action, Configured, Static},
    containers::{Context, Options},
};
pub use log_level::LogLevel;

pub struct Cli {
    ctx: Context,
    act: Action,
    opts: Options,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let (ctx, act, opts) = Args::parse().setup()?;
        Ok(Self { ctx, act, opts })
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub async fn run(self) -> JfResult<()> {
        match self.act {
            Action::Configured(act) => {
                let cfg = cfg::Cfg::load(self.opts.cfg)?;
                let jc = job_controller::JobController::new(cfg)?;
                match act {
                    Configured::List => {
                        println!("{}", jc.list().join(" "));
                    }
                    Configured::Validate => {
                        jc.validate()?;
                    }
                    Configured::Run { job_name } => {
                        jc.run(job_name).await?;
                    }
                    Configured::Description { job_name } => {
                        println!("{}", jc.description(job_name)?)
                    }
                }
            }
            Action::Static(act) => match act {
                Static::Help => {
                    <Args as clap::CommandFactory>::command().print_help()?;
                }
                Static::Completion { shell } => {
                    println!("{}", completion_script::generate(shell))
                }
            },
        }
        Ok(())
    }
}
