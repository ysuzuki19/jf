use crate::{
    common,
    config::CmdConfig,
    error::CmdResult,
    task::Runner,
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
            .map(Taskdef::try_from)
            .collect::<CmdResult<Vec<_>>>()?;
        Ok(Self {
            pool: TaskdefPool::new(task_vec),
        })
    }

    pub async fn run(&self, task_name: String) -> CmdResult<()> {
        self.pool
            .build(task_name, common::Agent::Cli)?
            .run()
            .await?
            .wait()
            .await?
            .kill()
            .await
    }

    pub fn description(&self, task_name: String) -> CmdResult<String> {
        self.pool.description(task_name)
    }
}
