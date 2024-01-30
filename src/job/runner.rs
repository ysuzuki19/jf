use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{
    ctx::{logger::LogWriter, Ctx},
    error::JfResult,
};

pub async fn sleep() {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

#[async_trait::async_trait]
pub trait Runner<LR: LogWriter>
where
    Self: Sized + Clone,
{
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self>;
    async fn is_finished(&self) -> JfResult<bool>;
    async fn cancel(&self) -> JfResult<Self>;
    fn bunshin(&self) -> Self;

    async fn join(&self) -> JfResult<Self> {
        loop {
            if self.is_finished().await? {
                break;
            }

            sleep().await;
        }
        Ok(self.clone())
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
