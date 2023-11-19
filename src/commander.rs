use crate::{
    cfg::CmdCfg,
    error::CmdResult,
    task::Runner,
    taskdef::{Agent, TaskdefPool},
};

pub struct Commander {
    pool: TaskdefPool,
}

impl Commander {
    pub fn new(cmd_cfg: CmdCfg) -> CmdResult<Self> {
        let task_vec = cmd_cfg
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
}
