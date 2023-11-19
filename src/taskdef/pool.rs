use std::{collections::HashMap, sync::Arc};

use super::Taskdef;
use crate::{
    common::{Agent, BuildContext},
    error::{CmdError, CmdResult},
    task::Task,
};

#[derive(Clone)]
pub struct TaskdefPool {
    map: Arc<HashMap<String, Taskdef>>,
}

impl TaskdefPool {
    pub fn new(taskdef_vec: Vec<Taskdef>) -> Self {
        let mut map = HashMap::new();
        for taskdef in taskdef_vec {
            map.insert(taskdef.name(), taskdef);
        }
        Self { map: Arc::new(map) }
    }

    fn ctx(&self) -> BuildContext {
        BuildContext::from(self.clone())
    }

    fn get(&self, task_name: String) -> CmdResult<&Taskdef> {
        self.map
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
