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
    async fn cancel(&self) -> CmdResult<()>;
    fn bunshin(&self) -> Self;

    async fn sleep() {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    async fn wait(&self) -> CmdResult<()> {
        loop {
            if self.is_finished().await? {
                break;
            }

            Self::sleep().await;
        }
        Ok(())
    }

    async fn wait_with_cancel(&self, is_cancelled: Arc<AtomicBool>) -> CmdResult<()> {
        loop {
            if self.is_finished().await? {
                break;
            }

            if is_cancelled.load(Ordering::Relaxed) {
                self.cancel().await?;
                break;
            }

            Self::sleep().await;
        }
        Ok(())
    }
}
