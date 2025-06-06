// SPDX-License-Identifier: MPL-2.0
mod canceller;
mod finish_notify;
mod join_status;
pub mod modes;
mod runner;
#[cfg(test)]
mod tests;

use futures::{stream, StreamExt};

pub use self::runner::*;
use self::{canceller::Canceller, join_status::JoinStatus};
use crate::{cfg::job_cfg::JobCfg, ctx::Ctx, jobdef::JobdefPool, util::error::JfResult};

#[derive(Clone)]
pub enum Job {
    Command(modes::Command),
    Parallel(modes::Parallel),
    Sequential(modes::Sequential),
    Shell(modes::Shell),
    Watch(modes::Watch),
    #[cfg(test)]
    Mock(modes::Mock),
}

impl Job {
    pub fn new(ctx: Ctx, job_cfg: &JobCfg, pool: JobdefPool) -> JfResult<Self> {
        Ok(match job_cfg {
            JobCfg::Command(c) => modes::Command::new(ctx, c.params.clone()).into(),
            JobCfg::Parallel(c) => modes::Parallel::new(ctx, c.params.clone(), pool)?.into(),
            JobCfg::Sequential(c) => modes::Sequential::new(ctx, c.params.clone(), pool)?.into(),
            JobCfg::Shell(c) => modes::Shell::new(ctx, c.params.clone()).into(),
            JobCfg::Watch(c) => modes::Watch::new(ctx, c.params.clone(), pool)?.into(),
            #[cfg(test)]
            JobCfg::Mock(c) => modes::Mock::new(c.params.clone()).into(),
        })
    }
}

#[async_trait::async_trait]
impl Bunshin for Job {
    async fn bunshin(&self) -> Self {
        match self {
            Self::Command(t) => Self::Command(t.bunshin().await),
            Self::Parallel(t) => Self::Parallel(t.bunshin().await),
            Self::Sequential(t) => Self::Sequential(t.bunshin().await),
            Self::Shell(t) => Self::Shell(t.bunshin().await),
            Self::Watch(t) => Self::Watch(t.bunshin().await),
            #[cfg(test)]
            Self::Mock(t) => Self::Mock(t.bunshin().await),
        }
    }
}

#[async_trait::async_trait]
impl Checker for Job {
    async fn is_finished(&self) -> JfResult<bool> {
        match self {
            Self::Command(t) => t.is_finished().await,
            Self::Parallel(t) => t.is_finished().await,
            Self::Sequential(t) => t.is_finished().await,
            Self::Shell(t) => t.is_finished().await,
            Self::Watch(t) => t.is_finished().await,
            #[cfg(test)]
            Self::Mock(t) => t.is_finished().await,
        }
    }
}

#[async_trait::async_trait]
impl Runner for Job {
    async fn start(&self) -> JfResult<Self> {
        Ok(match self {
            Self::Command(t) => t.start().await?.into(),
            Self::Parallel(t) => t.start().await?.into(),
            Self::Sequential(t) => t.start().await?.into(),
            Self::Shell(t) => t.start().await?.into(),
            Self::Watch(t) => t.start().await?.into(),
            #[cfg(test)]
            Self::Mock(t) => t.start().await?.into(),
        })
    }

    async fn cancel(&self) -> JfResult<Self> {
        Ok(match self {
            Self::Command(t) => t.cancel().await?.into(),
            Self::Parallel(t) => t.cancel().await?.into(),
            Self::Sequential(t) => t.cancel().await?.into(),
            Self::Shell(t) => t.cancel().await?.into(),
            Self::Watch(t) => t.cancel().await?.into(),
            #[cfg(test)]
            Self::Mock(t) => t.cancel().await?.into(),
        })
    }

    async fn join(&self) -> JfResult<JoinStatus> {
        match self {
            Self::Command(t) => t.join().await,
            Self::Parallel(t) => t.join().await,
            Self::Sequential(t) => t.join().await,
            Self::Shell(t) => t.join().await,
            Self::Watch(t) => t.join().await,
            #[cfg(test)]
            Self::Mock(t) => t.join().await,
        }
    }

    fn set_canceller(&mut self, canceller: Canceller) -> Self {
        match self {
            Self::Command(t) => Self::Command(t.set_canceller(canceller)),
            Self::Parallel(t) => Self::Parallel(t.set_canceller(canceller)),
            Self::Sequential(t) => Self::Sequential(t.set_canceller(canceller)),
            Self::Shell(t) => Self::Shell(t.set_canceller(canceller)),
            Self::Watch(t) => Self::Watch(t.set_canceller(canceller)),
            #[cfg(test)]
            Self::Mock(t) => Self::Mock(t.set_canceller(canceller)),
        }
    }
}

#[async_trait::async_trait]
impl Bunshin for Vec<Job> {
    async fn bunshin(&self) -> Self {
        stream::iter(self.iter())
            .then(|j| async { j.bunshin().await })
            .collect::<Vec<Job>>()
            .await
    }
}
