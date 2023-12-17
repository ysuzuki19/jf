pub mod modes;
mod runner;
mod types;

pub use self::runner::Runner;
use crate::{cfg::job_cfg::JobCfg, error::JfResult, jobdef::JobdefPool};

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
    pub fn new(job_cfg: JobCfg, pool: JobdefPool) -> JfResult<Self> {
        Ok(match job_cfg {
            JobCfg::Command(c) => modes::Command::new(c.params).into(),
            JobCfg::Parallel(c) => modes::Parallel::new(c.params, pool)?.into(),
            JobCfg::Sequential(c) => modes::Sequential::new(c.params, pool)?.into(),
            JobCfg::Shell(c) => modes::Shell::new(c.params).into(),
            JobCfg::Watch(c) => modes::Watch::new(c.params, pool)?.into(),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Job {
    async fn run(&self) -> JfResult<Self> {
        Ok(match self {
            Self::Command(t) => t.run().await?.into(),
            Self::Parallel(t) => t.run().await?.into(),
            Self::Sequential(t) => t.run().await?.into(),
            Self::Shell(t) => t.run().await?.into(),
            Self::Watch(t) => t.run().await?.into(),
            #[cfg(test)]
            Self::Mock(t) => t.run().await?.into(),
        })
    }

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

    async fn cancel(&self) -> JfResult<()> {
        match self {
            Self::Command(t) => t.cancel().await,
            Self::Parallel(t) => t.cancel().await,
            Self::Sequential(t) => t.cancel().await,
            Self::Shell(t) => t.cancel().await,
            Self::Watch(t) => t.cancel().await,
            #[cfg(test)]
            Self::Mock(t) => t.cancel().await,
        }
    }

    fn bunshin(&self) -> Self {
        match self {
            Self::Command(t) => Self::Command(t.bunshin()),
            Self::Parallel(t) => Self::Parallel(t.bunshin()),
            Self::Sequential(t) => Self::Sequential(t.bunshin()),
            Self::Shell(t) => Self::Shell(t.bunshin()),
            Self::Watch(t) => Self::Watch(t.bunshin()),
            #[cfg(test)]
            Self::Mock(t) => Self::Mock(t.bunshin()),
        }
    }
}
