use crate::error::CmdResult;
use crate::task::Task;
use crate::taskdef::TaskdefPool;

use super::Agent;

#[derive(Clone)]
pub struct BuildContext {
    pool: TaskdefPool,
}

impl BuildContext {
    pub fn new(pool: TaskdefPool) -> Self {
        Self { pool }
    }
}

impl BuildContext {
    pub fn build(&self, task_name: String) -> CmdResult<Task> {
        self.pool.build(task_name, Agent::Task)
    }
}
