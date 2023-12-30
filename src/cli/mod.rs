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
    context: Context,
    action: Action,
    options: Options,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let (context, action, options) = Args::parse().setup()?;
        Ok(Self {
            context,
            action,
            options,
        })
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub async fn run(self) -> JfResult<()> {
        match self.action {
            Action::Configured(action) => {
                let cfg = cfg::Cfg::load(self.options.cfg)?;
                let jc = job_controller::JobController::new(cfg)?;
                match action {
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
            Action::Static(action) => match action {
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
