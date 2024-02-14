use std::sync::{atomic::AtomicBool, Arc};

use crate::util::error::JfResult;

use super::join_status::JoinStatus;

pub(super) type JfHandle = tokio::task::JoinHandle<crate::util::error::JfResult<JoinStatus>>;

pub(super) const INTERVAL_MILLIS: u64 = 100;
pub(super) async fn interval() {
    tokio::time::sleep(tokio::time::Duration::from_millis(INTERVAL_MILLIS)).await;
}

#[async_trait::async_trait]
pub trait Bunshin {
    async fn bunshin(&self) -> Self;
}

#[async_trait::async_trait]
pub trait Checker {
    async fn is_finished(&self) -> JfResult<bool>;
}

#[async_trait::async_trait]
pub trait Runner: Checker + Bunshin
where
    Self: Sized + Clone,
{
    async fn start(&self) -> JfResult<Self>;
    async fn cancel(&self) -> JfResult<Self>;

    async fn reset(&mut self) -> JfResult<Self> {
        *self = self.bunshin().await;
        Ok(self.clone())
    }

    fn link_cancel(&mut self, _: Arc<AtomicBool>) -> Self {
        self.clone()
    }

    async fn pre_join(&self) -> JfResult<JoinStatus> {
        Ok(JoinStatus::Succeed)
    }
    async fn join(&self) -> JfResult<JoinStatus> {
        loop {
            if self.is_finished().await? {
                return self.pre_join().await;
            }

            // if self.is_cancelled() {
            //     self.cancel().await?;
            //     if !self.pre_join().await? {
            //         self.join().await?;
            //         return Ok(false);
            //     }
            //     return self.join().await;
            // }

            interval().await;
        }
    }
}
