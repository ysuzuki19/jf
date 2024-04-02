// SPDX-License-Identifier: MPL-2.0
#[cfg(test)]
mod tests;
mod watcher;

use std::{ops::DerefMut, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    ctx::Ctx,
    job::{canceller::Canceller, join_status::JoinStatus, runner::*, Job},
    jobdef::{Agent, JobdefPool},
    util::error::JfResult,
};

#[derive(Clone, serde::Deserialize)]
pub struct WatchParams {
    pub job: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch {
    ctx: Ctx,
    job: Arc<Mutex<Job>>,
    watch_list: Vec<String>,
    canceller: Canceller,
    handle: Arc<Mutex<Option<JfHandle>>>,
}

impl Watch {
    pub fn new(ctx: Ctx, params: WatchParams, pool: JobdefPool) -> JfResult<Self> {
        let job = pool.build(ctx.clone(), params.job, Agent::Job)?;
        Ok(Self {
            ctx,
            job: Arc::new(Mutex::new(job)),
            watch_list: params.watch_list,
            canceller: Canceller::new(),
            handle: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Bunshin for Watch {
    async fn bunshin(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            job: Arc::new(Mutex::new(self.job.lock().await.bunshin().await)),
            watch_list: self.watch_list.clone(),
            canceller: Canceller::new(),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl Checker for Watch {
    async fn is_finished(&self) -> JfResult<bool> {
        Ok(self.canceller.is_canceled())
    }
}

#[async_trait::async_trait]
impl Runner for Watch {
    async fn start(&self) -> JfResult<Self> {
        let mut logger = self.ctx.logger();
        logger.debug("Watch starting...").await?;
        let handle = tokio::spawn({
            let watch_list = self.watch_list.clone();
            let job = self.job.clone();
            let canceller = self.canceller.clone();
            job.lock().await.start().await?;

            async move {
                loop {
                    watcher::JfWatcher::new(&watch_list, canceller.clone())?
                        .wait()
                        .await?;

                    job.lock().await.cancel().await?.join().await?;
                    if canceller.is_canceled() {
                        return Ok(JoinStatus::Failed);
                    }

                    job.lock().await.reset().await?.start().await?;
                }
            }
        });
        self.handle.lock().await.replace(handle);
        logger.debug("Watch started").await?;
        Ok(self.clone())
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.canceller.cancel();
        self.job.lock().await.cancel().await?.join().await?;
        Ok(self.clone())
    }

    async fn join(&self) -> JfResult<JoinStatus> {
        loop {
            if self.is_finished().await? {
                if let Some(handle) = self.handle.lock().await.deref_mut() {
                    return handle.await?;
                }
            }

            interval().await;
        }
    }

    fn set_canceller(&mut self, canceller: Canceller) -> Self {
        self.canceller = canceller;
        self.clone()
    }
}

impl From<Watch> for Job {
    fn from(value: Watch) -> Self {
        Self::Watch(value)
    }
}
