use crate::common;
use crate::taskdef::pool::TaskdefPool;

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
    pub fn build(&self, task_name: String) -> crate::error::CmdResult<crate::task::Task> {
        self.pool.build(task_name, common::Agent::Task)
    }
}
