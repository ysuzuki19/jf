use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::error::CmdResult;

#[async_trait::async_trait]
pub trait Runner
where
    Self: Sized + Clone,
{
    async fn run(&self) -> CmdResult<Self>;
    async fn is_finished(&self) -> CmdResult<bool>;
    async fn wait(&self) -> CmdResult<Self> {
        loop {
            if self.is_finished().await? {
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        Ok(self.clone())
    }

    async fn wait_with_cancel(&self, is_cancelled: Arc<AtomicBool>) -> CmdResult<Self> {
        loop {
            if self.is_finished().await? {
                break;
            }

            if is_cancelled.load(Ordering::Relaxed) {
                self.cancel().await?;
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        Ok(self.clone())
    }

    async fn cancel(&self) -> CmdResult<()>;
    fn bunshin(&self) -> Self;
}
