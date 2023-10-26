use crate::error::CmdResult;

#[async_trait::async_trait]
pub trait Runner {
    async fn run(&self) -> CmdResult<()>;
    async fn is_finished(&self) -> CmdResult<bool>;
    async fn wait(&self) -> CmdResult<()> {
        loop {
            if self.is_finished().await? {
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        Ok(())
    }
    async fn kill(self) -> CmdResult<()>;
}
