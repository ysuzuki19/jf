mod log_driver;

use crate::{
    ctx::{logger::LogWriter, Ctx},
    error::{InternalError, JfResult},
};

pub struct CommandDriver<LR: LogWriter> {
    child: tokio::process::Child,
    log_driver: log_driver::LogDriver<LR>,
}

impl<LR: LogWriter> CommandDriver<LR> {
    pub async fn spawn(ctx: Ctx<LR>, command: &String, args: &Vec<String>) -> JfResult<Self> {
        let mut cmd = tokio::process::Command::new(command);
        cmd.args(args);
        cmd.stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn()?;
        let mut log_driver = log_driver::LogDriver::new(ctx);

        match log_driver.mount(child.stdout.take()) {
            Ok(_) => Ok(Self { child, log_driver }),
            Err(_) => {
                child.kill().await?;
                Err(InternalError::FailedToHandleStdout(command.to_owned()).into())
            }
        }
    }

    pub async fn is_finished(&mut self) -> JfResult<bool> {
        Ok(self.child.try_wait()?.is_some())
    }

    pub async fn cancel(&mut self) -> JfResult<()> {
        if let Err(e) = self.child.kill().await {
            match e.kind() {
                std::io::ErrorKind::InvalidInput => {}
                _ => return Err(e.into()),
            }
        }
        self.log_driver.join().await?;
        Ok(())
    }
}
