use crate::{
    common,
    config::CmdConfig,
    error::CmdResult,
    task::runner::Runner,
    taskdef::{Taskdef, TaskdefPool},
};

pub struct Commander {
    pool: TaskdefPool,
}

impl Commander {
    pub fn new(config: CmdConfig) -> CmdResult<Self> {
        let task_vec = config
            .tasks
            .into_iter()
            .map(|(name, task_config)| Taskdef::new(name, task_config))
            .collect::<CmdResult<Vec<_>>>()?;
        Ok(Self {
            pool: TaskdefPool::new(task_vec),
        })
    }

    pub async fn run(&self, task_name: String) -> CmdResult<()> {
        let task = self.pool.build(task_name, common::Agent::Cli)?;
        task.run().await?;
        task.wait().await?;
        Ok(())
    }

    pub fn description(&self, task_name: String) -> CmdResult<String> {
        self.pool.description(task_name)
    }
}
