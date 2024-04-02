// SPDX-License-Identifier: MPL-2.0
use crate::util::error::JfResult;

use super::{canceller::Canceller, join_status::JoinStatus};

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
    async fn join(&self) -> JfResult<JoinStatus>;

    fn set_canceller(&mut self, _: Canceller) -> Self;

    async fn reset(&mut self) -> JfResult<Self> {
        *self = self.bunshin().await;
        Ok(self.clone())
    }
}
