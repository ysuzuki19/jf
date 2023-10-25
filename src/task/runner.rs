use crate::error::CmdResult;

#[async_trait::async_trait]
pub trait Runner {
    async fn run(&self) -> CmdResult<()>;
    async fn wait(&self) -> CmdResult<()>;
    async fn kill(self) -> CmdResult<()>;
}
