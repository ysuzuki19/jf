mod pool;

use crate::{
    common::{Agent, BuildContext},
    error::{CmdError, CmdResult},
};
pub use pool::TaskdefPool;

pub struct Taskdef {
    pub(super) name: String,
    private: bool,
    description: String,
    task_config: crate::config::TaskConfig,
}

impl Taskdef {
    pub fn new(name: String, task_config: crate::config::TaskConfig) -> CmdResult<Self> {
        Ok(Self {
            name,
            private: task_config.private(),
            description: task_config.description(),
            task_config,
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

    fn build(&self, bc: BuildContext, agent: Agent) -> CmdResult<crate::task::Task> {
        self.visibility_guard(agent)?;
        crate::task::Task::new(self.task_config.clone(), bc)
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}
