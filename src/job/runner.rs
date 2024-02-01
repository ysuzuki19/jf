use std::sync::{atomic::AtomicBool, Arc};

use crate::util::error::JfResult;

pub(super) type JfHandle = tokio::task::JoinHandle<crate::util::error::JfResult<()>>;

pub(super) const INTERVAL_MILLIS: u64 = 100;
pub(super) async fn interval() {
    tokio::time::sleep(tokio::time::Duration::from_millis(INTERVAL_MILLIS)).await;
}

#[async_trait::async_trait]
pub(super) trait Bunshin {
    async fn bunshin(&self) -> Self;
}

#[async_trait::async_trait]
pub trait Checker {
    async fn is_finished(&self) -> JfResult<bool>;
    fn is_cancelled(&self) -> bool {
        false
    }
}

#[async_trait::async_trait]
pub trait Runner: Checker
where
    Self: Sized + Clone,
{
    async fn start(&self) -> JfResult<Self>;
    async fn cancel(&self) -> JfResult<Self>;

    fn link_cancel(&mut self, _: Arc<AtomicBool>) -> Self {
        self.clone()
    }

    async fn pre_join(&self) -> JfResult<()> {
        Ok(())
    }
    async fn join(&self) -> JfResult<Self> {
        loop {
            if self.is_finished().await? {
                break;
            }

            if self.is_cancelled() {
                self.cancel().await?;
                self.pre_join().await?;
                self.join().await?;
                break;
            }

            interval().await;
        }
        Ok(self.clone())
    }
}
