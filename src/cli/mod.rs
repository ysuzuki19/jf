mod args;
mod behavior;
mod completion_script;
mod job_controller;

use clap::Parser;

use crate::{cfg, error::JfResult};

pub use self::args::Args;
use self::behavior::{CliBehavior, Configured, Static};

pub struct Cli {
    args: Args,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let args = Args::parse();
        Ok(Self { args })
    }

    pub fn error_log_enabled(&self) -> bool {
        !self.args.list
    }

    pub async fn run(mut self) -> JfResult<()> {
        let cfg_option = self.args.cfg.take();
        match self.args.try_into()? {
            CliBehavior::Configured(wjc) => {
                let cfg = cfg::Cfg::load(cfg_option)?;
                let jc = job_controller::JobController::new(cfg)?;
                match wjc {
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
            CliBehavior::Static(Static::Help) => {
                <Args as clap::CommandFactory>::command().print_help()?;
            }
            CliBehavior::Static(Static::Completion { shell }) => {
                println!("{}", completion_script::generate(shell))
            }
        }
        Ok(())
    }
}
