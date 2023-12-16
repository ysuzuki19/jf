mod args;
mod commander;
mod completion_script;

use clap::Parser;

use crate::{cfg, error::JfResult};

use self::{
    args::{Args, SubCommand},
    commander::Commander,
};

pub struct Cli {
    args: Args,
    commander: Commander,
}

impl Cli {
    pub fn load() -> JfResult<Self> {
        let args = Args::parse();
        let cfg = cfg::Cfg::load()?;
        let commander = commander::Commander::new(cfg)?;
        Ok(Self { args, commander })
    }

    pub async fn run(self) -> JfResult<()> {
        if let Some(sub_command) = self.args.sub_command {
            match sub_command {
                SubCommand::Completion { shell } => {
                    println!("{}", completion_script::generate(shell))
                }
                SubCommand::Run { task_name } => {
                    self.commander.run(task_name).await?;
                }
                SubCommand::Description { task_name } => {
                    println!("{}", self.commander.description(task_name)?);
                }
                SubCommand::List => {
                    println!("{}", self.commander.list().join("\n"));
                }
            }
        }
        Ok(())
    }
}
