pub mod modes;
mod runner;
#[cfg(test)]
mod tests;

use futures::{stream, StreamExt};

pub use self::runner::*;
use crate::{
    cfg::job_cfg::JobCfg,
    ctx::{logger::LogWriter, Ctx},
    jobdef::JobdefPool,
    util::error::JfResult,
};

#[derive(Clone)]
pub enum Job<LR: LogWriter> {
    Command(modes::Command<LR>),
    Parallel(modes::Parallel<LR>),
    Sequential(modes::Sequential<LR>),
    Shell(modes::Shell<LR>),
    Watch(modes::Watch<LR>),
    #[cfg(test)]
    Mock(modes::Mock<LR>),
}

impl<LR: LogWriter> Job<LR> {
    pub fn new(job_cfg: &JobCfg, pool: JobdefPool) -> JfResult<Self> {
        Ok(match job_cfg {
            JobCfg::Command(c) => modes::Command::new(c.params.clone()).into(),
            JobCfg::Parallel(c) => modes::Parallel::new(c.params.clone(), pool)?.into(),
            JobCfg::Sequential(c) => modes::Sequential::new(c.params.clone(), pool)?.into(),
            JobCfg::Shell(c) => modes::Shell::new(c.params.clone()).into(),
            JobCfg::Watch(c) => modes::Watch::new(c.params.clone(), pool)?.into(),
            #[cfg(test)]
            JobCfg::Mock(c) => modes::Mock::new(c.params.clone()).into(),
        })
    }
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Job<LR> {
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
impl<LR: LogWriter> Checker for Job<LR> {
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
impl<LR: LogWriter> Runner<LR> for Job<LR> {
    async fn start(&self, ctx: Ctx<LR>) -> JfResult<Self> {
        Ok(match self {
            Self::Command(t) => t.start(ctx).await?.into(),
            Self::Parallel(t) => t.start(ctx).await?.into(),
            Self::Sequential(t) => t.start(ctx).await?.into(),
            Self::Shell(t) => t.start(ctx).await?.into(),
            Self::Watch(t) => t.start(ctx).await?.into(),
            #[cfg(test)]
            Self::Mock(t) => t.start(ctx).await?.into(),
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
}

#[async_trait::async_trait]
impl<LR: LogWriter> Bunshin for Vec<Job<LR>> {
    async fn bunshin(&self) -> Self {
        stream::iter(self.iter())
            .then(|j| async { j.bunshin().await })
            .collect::<Vec<Job<LR>>>()
            .await
    }
}
