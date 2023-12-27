mod args;
mod behavior;
mod completion_script;
mod job_controller;
mod log_level;

use clap::Parser;

use crate::{cfg, error::JfResult, LOG_LEVEL};

pub use self::args::Args;
use self::behavior::{CliBehavior, Configured, Static};
pub use log_level::LogLevel;

pub struct Cli {
    args: Args,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let args = Args::parse();
        Ok(Self { args })
    }

    async fn load_log_level(&self) {
        *LOG_LEVEL.write().await = if self.args.list {
            LogLevel::None
        } else {
            self.args.log_level.clone()
        }
    }

    pub async fn run(mut self) -> JfResult<()> {
        self.load_log_level().await;
        let cfg_option = self.args.cfg.take();
        match self.args.try_into()? {
            CliBehavior::Configured(behavior) => {
                let cfg = cfg::Cfg::load(cfg_option)?;
                let jc = job_controller::JobController::new(cfg)?;
                match behavior {
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
            CliBehavior::Static(behavior) => match behavior {
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
