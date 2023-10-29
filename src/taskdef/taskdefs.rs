use std::{collections::HashMap, sync::Arc};

use crate::{
    common,
    error::{CmdError, CmdResult},
    task::Task,
};

use super::Taskdef;

#[derive(Clone)]
pub struct TaskPool {
    map: Arc<HashMap<String, Taskdef>>,
}

impl TaskPool {
    pub fn new(task_vec: Vec<Taskdef>) -> CmdResult<Self> {
        let mut map = HashMap::new();
        for task in task_vec {
            map.insert(task.name.clone(), task);
        }
        Ok(Self { map: Arc::new(map) })
    }

    fn ctx(&self) -> super::context::Context {
        super::Context::new(self.clone())
    }

    pub fn get(&self, task_name: String) -> CmdResult<&Taskdef> {
        self.map
            .get(&task_name)
            .ok_or(CmdError::TaskdefNotFound(task_name))
    }

    pub fn build(&self, task_name: String, agent: common::Agent) -> CmdResult<Task> {
        self.get(task_name)?.build(self.ctx(), agent)
    }

    pub fn description(&self, task_name: String) -> CmdResult<String> {
        Ok(self.get(task_name)?.description())
    }
}
