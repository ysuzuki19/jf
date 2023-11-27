use crate::{
    cfg::Cfg,
    error::CmdResult,
    task::Runner,
    taskdef::{Agent, TaskdefPool},
};

pub struct Commander {
    pool: TaskdefPool,
}

impl Commander {
    pub fn new(cfg: Cfg) -> CmdResult<Self> {
        let task_vec = cfg
            .tasks
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<CmdResult<_>>()?;
        Ok(Self {
            pool: TaskdefPool::new(task_vec),
        })
    }

    pub async fn run(&self, task_name: String) -> CmdResult<()> {
        self.pool
            .build(task_name, Agent::Cli)?
            .run()
            .await?
            .wait()
            .await
    }

    pub fn description(&self, task_name: String) -> CmdResult<String> {
        self.pool.description(task_name)
    }

    pub fn list(&self) -> Vec<String> {
        let mut task_names = self.pool.list();
        task_names.sort();
        task_names
    }
}
