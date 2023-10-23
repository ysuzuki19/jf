use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::{runner::Runner, Agent, Context},
};

use super::Run;

#[derive(Clone)]
pub struct Parallel {
    pub tasks: Vec<String>,
    // pub handles: Arc<Mutex<Vec<>
}

#[async_trait::async_trait]
impl Run for Parallel {
    async fn run(&mut self, ctx: Context) -> CmdResult<()> {
        let mut handles = Vec::new();
        for task in self.tasks.clone() {
            println!("Running... {}", task.clone());
            handles.push(ctx.tasks.get_and_run(task, Agent::Task));
        }

        for handle in handles {
            handle.await?;
        }

        Ok(())
    }
}

impl Parallel {
    pub fn from_config(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let tasks = runner_config
            .tasks
            .ok_or_else(|| CmdError::TaskdefMissingField("sequential".into(), "tasks".into()))?;
        Ok(Self { tasks })
    }

    pub async fn kill(self) -> CmdResult<()> {
        // if let Some(child) = self.clone().child {
        //     child.lock().await.kill().await?;
        // }
        todo!();
        Ok(())
    }
}

impl From<Parallel> for Runner {
    fn from(value: Parallel) -> Self {
        Runner::Parallel(value)
    }
}
