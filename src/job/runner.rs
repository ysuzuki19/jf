use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::error::JfResult;

pub async fn sleep() {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

#[async_trait::async_trait]
pub trait Runner
where
    Self: Sized + Clone,
{
    async fn start(&self) -> JfResult<Self>;
    async fn is_finished(&self) -> JfResult<bool>;
    async fn cancel(&self) -> JfResult<Self>;
    fn bunshin(&self) -> Self;

    async fn wait(&self) -> JfResult<()> {
        loop {
            if self.is_finished().await? {
                break;
            }

            sleep().await;
        }
        Ok(())
    }

    async fn wait_with_cancel(&self, is_cancelled: Arc<AtomicBool>) -> JfResult<()> {
        loop {
            if self.is_finished().await? {
                break;
            }

            if is_cancelled.load(Ordering::Relaxed) {
                self.cancel().await?;
                break;
            }

            sleep().await;
        }
        Ok(())
    }
}
