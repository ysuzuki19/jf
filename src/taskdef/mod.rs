mod pool;

pub use self::pool::TaskdefPool;
use crate::{
    cfg::TaskCfg,
    common::{Agent, BuildContext},
    error::{CmdError, CmdResult},
    task::Task,
};

pub struct Taskdef {
    name: String,
    private: bool,
    description: String,
    task_config: TaskCfg,
}

impl Taskdef {
    pub fn new(name: String, task_config: TaskCfg) -> CmdResult<Self> {
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

    fn build(&self, bc: BuildContext, agent: Agent) -> CmdResult<Task> {
        self.visibility_guard(agent)?;
        Task::new(self.task_config.clone(), bc)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl TryFrom<(String, TaskCfg)> for Taskdef {
    type Error = CmdError;

    fn try_from(value: (String, TaskCfg)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}
