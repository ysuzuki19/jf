use crate::{
    config::CmdConfig,
    error::CmdResult,
    task::runner::Runner,
    taskdef::{taskdefs::Taskdefs, Taskdef},
};

pub struct Commander {
    task_defs: Taskdefs,
}

impl Commander {
    pub fn new(config: CmdConfig) -> CmdResult<Self> {
        let task_vec = config
            .tasks
            .into_iter()
            .map(|(name, task_config)| Taskdef::new(name, task_config))
            .collect::<CmdResult<Vec<_>>>()?;
        Ok(Self {
            task_defs: Taskdefs::new(task_vec)?,
        })
    }

    pub async fn run(&self, task_name: String) -> CmdResult<()> {
        let task = self.task_defs.build(task_name, crate::task::Agent::Cli)?;
        task.run().await?;
        task.wait().await?;
        Ok(())
    }

    pub fn description(&self, task_name: String) -> CmdResult<String> {
        self.task_defs.description(task_name)
    }
}
