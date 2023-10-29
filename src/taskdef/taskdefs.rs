use std::{collections::HashMap, sync::Arc};

use crate::{
    error::{CmdError, CmdResult},
    task::{Agent, Task},
};

use super::Taskdef;

#[derive(Clone)]
pub struct Taskdefs {
    tasks: Arc<HashMap<String, Taskdef>>,
}

impl Taskdefs {
    pub fn new(task_vec: Vec<Taskdef>) -> CmdResult<Self> {
        let mut task_map = HashMap::new();
        for task in task_vec {
            task_map.insert(task.name.clone(), task);
        }
        Ok(Self {
            tasks: Arc::new(task_map),
        })
    }

    fn ctx(&self) -> super::context::Context {
        super::Context::new(self.clone())
    }

    pub fn get(&self, task_name: String) -> CmdResult<&Taskdef> {
        self.tasks
            .get(&task_name)
            .ok_or(CmdError::TaskdefNotFound(task_name))
    }

    pub fn build(&self, task_name: String, agent: Agent) -> CmdResult<Task> {
        self.get(task_name)?.build(self.ctx(), agent)
    }

    pub fn description(&self, task_name: String) -> CmdResult<String> {
        Ok(self.get(task_name)?.description())
    }
}
