use crate::common;

use super::taskdefs::TaskPool;

#[derive(Clone)]
pub struct Context {
    pool: TaskPool,
}

impl Context {
    pub fn new(pool: TaskPool) -> Self {
        Self { pool }
    }
}

impl Context {
    pub fn build(&self, task_name: String) -> crate::error::CmdResult<crate::task::Task> {
        self.pool.build(task_name, common::Agent::Task)
    }
}
