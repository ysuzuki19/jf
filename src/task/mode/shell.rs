use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    error::{CmdError, CmdResult},
    task::{runner::Runner, Context},
};

use super::Run;

#[derive(Clone)]
pub struct Shell {
    pub script: String,
    child: Option<Arc<Mutex<tokio::process::Child>>>,
}

#[async_trait::async_trait]
impl Run for Shell {
    async fn run(&mut self, _: Context) -> CmdResult<()> {
        let mut cmd = tokio::process::Command::new("sh");
        cmd.arg("-c").arg(self.script.clone());
        // let mut child = cmd.spawn()?;
        // child.kill().await?;
        // child.wait().await?;
        let child = cmd.spawn()?;
        self.child = Some(Arc::new(Mutex::new(child)));
        self.child.clone().unwrap().lock().await.wait().await?;
        Ok(())
    }
}

impl Shell {
    pub fn from_config(runner_config: crate::config::RunnerConfig) -> CmdResult<Self> {
        let script = runner_config
            .script
            .ok_or_else(|| CmdError::TaskdefMissingField("shell".into(), "script".into()))?;
        Ok(Self {
            script,
            child: None,
        })
    }

    pub async fn kill(self) -> CmdResult<()> {
        if let Some(child) = self.child {
            child.lock().await.kill().await?;
        }
        Ok(())
    }
}

impl From<Shell> for Runner {
    fn from(value: Shell) -> Self {
        Runner::Shell(value)
    }
}
