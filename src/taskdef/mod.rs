pub mod pool;

use crate::common;
use crate::error::{CmdError, CmdResult};

pub struct Taskdef {
    pub(super) name: String,
    private: bool,
    description: String,
    runner_config: crate::config::RunnerConfig,
}

impl Taskdef {
    pub fn new(name: String, task_config: crate::config::TaskConfig) -> CmdResult<Self> {
        let (common_config, runner_config) = task_config.into_pruned();
        Ok(Self {
            name,
            private: common_config.private,
            description: common_config.description,
            runner_config,
        })
    }

    fn visibility_guard(&self, agent: common::Agent) -> CmdResult<()> {
        if !self.private {
            return Ok(());
        }
        match agent {
            common::Agent::Cli => Err(CmdError::Custom(format!(
                "task.{} is private\nPlease remove `private = true` if you run",
                self.name
            ))),
            _ => Ok(()),
        }
    }

    fn build(
        &self,
        bc: common::BuildContext,
        agent: common::Agent,
    ) -> CmdResult<crate::task::Task> {
        self.visibility_guard(agent)?;
        crate::task::Task::new(self.runner_config.clone(), bc)
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}
