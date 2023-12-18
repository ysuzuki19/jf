mod args;
mod commander;
mod completion_script;

use clap::Parser;

use crate::{cfg, error::JfResult};

pub use self::args::{Args, SubCommand};

pub struct Cli {
    args: Args,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let args = Args::parse();
        Ok(Self { args })
    }

    pub fn error_log_enabled(&self) -> bool {
        !matches!(self.args.sub_command, Some(SubCommand::List))
    }

    pub async fn run(self) -> JfResult<()> {
        if let Some(sub_command) = self.args.sub_command {
            match sub_command {
                SubCommand::Completion { shell } => {
                    println!("{}", completion_script::generate(shell))
                }
                _ => {
                    let cfg = if let Some(cfg_path) = self.args.cfg {
                        cfg::Cfg::load_with_path(&cfg_path)?
                    } else {
                        cfg::Cfg::load()?
                    };
                    let cmdr = commander::Commander::new(cfg)?;
                    match sub_command {
                        SubCommand::Run { job_name } => {
                            cmdr.run(job_name).await?;
                        }
                        SubCommand::Description { job_name } => {
                            println!("{}", cmdr.description(job_name)?);
                        }
                        SubCommand::List => {
                            println!("{}", cmdr.list().join(" "));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            <Args as clap::CommandFactory>::command().print_help()?;
        }
        Ok(())
    }
}
