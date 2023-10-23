use std::{collections::HashMap, sync::Arc};

use crate::error::{CmdError, CmdResult};

use super::{taskdef::Taskdef, Agent};

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

    fn ctx(&self) -> crate::task::runner::Context {
        crate::task::runner::Context {
            tasks: self.clone(),
        }
    }

    pub fn get(&self, task: &str) -> CmdResult<&Taskdef> {
        self.tasks
            .get(task)
            .ok_or(CmdError::TaskdefNotFound(task.into()))
    }

    pub(super) async fn get_and_run(&self, task: String, agent: Agent) -> CmdResult<()> {
        self.tasks
            .get(&task)
            .ok_or(CmdError::TaskdefNotFound(task))?
            .run(self.ctx(), agent)
            .await
    }

    /// run from cli
    pub async fn run(&self, task: String) -> CmdResult<()> {
        self.get_and_run(task, Agent::Cli).await
    }

    pub fn description(&self, task: &str) -> CmdResult<String> {
        Ok(self.get(task)?.description())
    }
}
