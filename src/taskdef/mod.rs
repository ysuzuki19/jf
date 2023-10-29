mod pool;

pub use self::pool::TaskdefPool;
use crate::{
    common::{Agent, BuildContext},
    error::{CmdError, CmdResult},
};

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

impl TryFrom<(String, crate::config::TaskConfig)> for Taskdef {
    type Error = CmdError;

    fn try_from(value: (String, crate::config::TaskConfig)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}
