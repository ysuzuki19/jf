pub mod modes;
mod runner;
mod types;

pub use self::runner::Runner;
use crate::{
    cfg::job_cfg::JobCfg,
    ctx::{logger::LogWriter, Ctx},
    error::JfResult,
    jobdef::JobdefPool,
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

#[cfg(test)]
mod tests {
    use crate::testutil::*;

    use self::modes::Command;

    use super::*;

    impl Job<MockLogWriter> {
        #[cfg_attr(coverage, coverage(off))]
        pub fn as_mock(&self) -> &modes::Mock<MockLogWriter> {
            if let Self::Mock(t) = self {
                t
            } else {
                panic!("invalid job type expected Mock");
            }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let job: Job<_> = Command::fixture().into();
                job.start(Ctx::fixture())
                    .await?
                    .cancel()
                    .await?
                    .wait()
                    .await?;
                assert!(job.is_finished().await?);
                let job = job.bunshin();
                assert!(!job.is_finished().await?);

                let job: Job<_> = modes::Parallel::try_fixture()?.into();
                job.start(Ctx::fixture())
                    .await?
                    .cancel()
                    .await?
                    .wait()
                    .await?;
                assert!(job.is_finished().await?);
                let job = job.bunshin();
                assert!(!job.is_finished().await?);

                let job: Job<_> = modes::Sequential::try_fixture()?.into();
                job.start(Ctx::fixture())
                    .await?
                    .cancel()
                    .await?
                    .wait()
                    .await?;
                assert!(job.is_finished().await?);
                let job = job.bunshin();
                assert!(!job.is_finished().await?);

                let job: Job<_> = modes::Shell::fixture().into();
                job.start(Ctx::fixture())
                    .await?
                    .cancel()
                    .await?
                    .wait()
                    .await?;
                assert!(job.is_finished().await?);
                let job = job.bunshin();
                assert!(!job.is_finished().await?);

                let job: Job<_> = modes::Watch::try_fixture()?.into();
                job.start(Ctx::fixture())
                    .await?
                    .cancel()
                    .await?
                    .wait()
                    .await?;
                assert!(job.is_finished().await?);
                let job = job.bunshin();
                assert!(!job.is_finished().await?);

                Ok(())
            },
        )
    }
}
