mod args;
mod completion_script;
mod job_controller;

use clap::Parser;

use crate::{cfg, error::JfResult};

pub use self::args::Args;
use self::args::{CliBehavior, Configured, Static};

pub struct Cli {
    args: Args,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let args = Args::parse();
        Ok(Self { args })
    }

    pub fn error_log_enabled(&self) -> bool {
        let res_cli_behavior = self.args.clone().try_into();
        match res_cli_behavior {
            Ok(cb) => !matches!(cb, CliBehavior::Configured(Configured::List)),
            Err(_) => true,
        }
    }

    pub async fn run(mut self) -> JfResult<()> {
        let cfg_option = self.args.cfg.take();
        match self.args.try_into()? {
            CliBehavior::Static(Static::Completion { shell }) => {
                println!("{}", completion_script::generate(shell))
            }
            CliBehavior::Static(Static::Help) => {
                <Args as clap::CommandFactory>::command().print_help()?;
            }
            CliBehavior::Configured(wjc) => {
                let cfg = cfg::Cfg::load(cfg_option)?;
                let jc = job_controller::JobController::new(cfg)?;
                match wjc {
                    Configured::List => {
                        println!("{}", jc.list().join(" "));
                    }
                    Configured::Description { job_name } => {
                        println!("{}", jc.description(job_name)?)
                    }
                    Configured::Run { job_name } => {
                        jc.run(job_name).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
