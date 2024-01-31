use std::sync::{atomic::AtomicBool, Arc};

use crate::{
    ctx::{logger::LogWriter, Ctx},
    util::error::JfResult,
};

pub async fn sleep() {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

#[async_trait::async_trait]
pub trait Bunshin {
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
pub trait Runner<LR: LogWriter>: Checker
where
    Self: Sized + Clone,
{
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self>;
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

            sleep().await;
        }
        Ok(self.clone())
    }
}
