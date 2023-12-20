mod args;
mod completion_script;
mod job_controller;

use clap::Parser;

use crate::{cfg, error::JfResult};

pub use self::args::Args;
use self::args::CliBehavior;

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
            Ok(cli_behavior) => !matches!(cli_behavior, CliBehavior::List),
            Err(_) => true,
        }
    }

    pub async fn run(mut self) -> JfResult<()> {
        let cfg_option = self.args.cfg.take();
        let cli_behavior = self.args.try_into()?;
        match cli_behavior {
            CliBehavior::Completion { shell } => {
                println!("{}", completion_script::generate(shell))
            }
            CliBehavior::Help => {
                <Args as clap::CommandFactory>::command().print_help()?;
            }
            _ => {
                let cfg = cfg::Cfg::load(cfg_option)?;
                let jc = job_controller::JobController::new(cfg)?;
                match cli_behavior {
                    CliBehavior::List => {
                        println!("{}", jc.list().join(" "));
                    }
                    CliBehavior::Description { job_name } => {
                        println!("{}", jc.description(job_name)?)
                    }
                    CliBehavior::Run { job_name } => {
                        jc.run(job_name).await?;
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}
