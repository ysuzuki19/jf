use crate::{
    config::CmdConfig,
    error::CmdResult,
    task::{Taskdef, Taskdefs},
};

// Layer for user control
pub struct Commander {
    tasks: Taskdefs,
}

impl Commander {
    pub fn new(config: CmdConfig) -> CmdResult<Self> {
        let task_vec = config
            .tasks
            .into_iter()
            .map(|(name, task_config)| Taskdef::new(name, task_config))
            .collect::<CmdResult<Vec<_>>>()?;
        Ok(Self {
            tasks: Taskdefs::new(task_vec)?,
        })
    }

    pub async fn run(&self, task: String) -> CmdResult<()> {
        self.tasks.run(task).await
    }

    pub fn description(&self, task: &str) -> CmdResult<String> {
        self.tasks.description(task)
    }
}
