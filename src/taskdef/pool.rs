use std::{collections::HashMap, sync::Arc};

use super::{Agent, Taskdef};
use crate::{
    error::{JfError, JfResult},
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

    fn get(&self, task_name: String) -> JfResult<&Taskdef> {
        self.map
            .get(&task_name)
            .ok_or(JfError::TaskdefNotFound(task_name))
    }

    pub fn list(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn build(&self, task_name: String, agent: Agent) -> JfResult<Task> {
        self.get(task_name)?.build(self.clone(), agent)
    }

    pub fn description(&self, task_name: String) -> JfResult<String> {
        Ok(self.get(task_name)?.description())
    }
}
