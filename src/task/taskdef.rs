use crate::error::{CmdError, CmdResult};

use super::{
    runner::{self, Runner},
    Agent,
};

pub struct Taskdef {
    pub(super) name: String,
    private: bool,
    description: String,
    runner: Runner,
}

impl Taskdef {
    pub fn new(name: String, task_config: crate::config::TaskConfig) -> CmdResult<Self> {
        let (common_config, task_config_pruned) = task_config.into_pruned();
        let runner = Runner::new(task_config_pruned).map_err(|e| {
            CmdError::TaskdefParse(name.clone(), format!("runner: {}", e.to_string()))
        })?;
        Ok(Self {
            name,
            runner,
            private: common_config.private,
            description: common_config.description,
        })
    }

    fn visibility_guard(&self, agent: Agent) -> CmdResult<()> {
        if !self.private {
            return Ok(());
        }
        match agent {
            Agent::Cli => Err(CmdError::Custom(format!(
                "task.{} is private\nPlease remove `private = true` if you run",
                self.name
            ))),
            _ => Ok(()),
        }
    }

    pub(super) async fn run(&self, ctx: runner::Context, agent: Agent) -> CmdResult<()> {
        self.visibility_guard(agent.clone())?;
        self.runner.clone().run(ctx).await
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}
