mod agent;
mod pool;

pub use self::agent::Agent;
pub use self::pool::TaskdefPool;
use crate::{
    cfg::TaskCfg,
    error::{JfError, JfResult},
    task::Task,
};

pub struct Taskdef {
    name: String,
    private: bool,
    description: String,
    task_cfg: TaskCfg,
}

impl Taskdef {
    pub fn new(name: String, task_cfg: TaskCfg) -> JfResult<Self> {
        Ok(Self {
            name,
            private: task_cfg.private(),
            description: task_cfg.description(),
            task_cfg,
        })
    }

    fn visibility_guard(&self, agent: Agent) -> JfResult<()> {
        if !self.private {
            return Ok(());
        }
        match agent {
            Agent::Cli => Err(JfError::Custom(format!(
                "task.{} is private\nPlease remove `private = true` if you run",
                self.name
            ))),
            _ => Ok(()),
        }
    }

    fn build(&self, pool: TaskdefPool, agent: Agent) -> JfResult<Task> {
        self.visibility_guard(agent)?;
        Task::new(self.task_cfg.clone(), pool)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl TryFrom<(String, TaskCfg)> for Taskdef {
    type Error = JfError;

    fn try_from(value: (String, TaskCfg)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}
