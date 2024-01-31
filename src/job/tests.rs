use crate::util::testutil::*;

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
fn command() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let job: Job<_> = modes::Command::fixture().into();
            job.start(Ctx::fixture())
                .await?
                .cancel()
                .await?
                .join()
                .await?;
            assert!(job.is_finished().await?);
            let job = job.bunshin().await;
            assert!(!job.is_finished().await?);

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn parallel() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let job: Job<_> = modes::Parallel::try_fixture()?.into();
            job.start(Ctx::fixture())
                .await?
                .cancel()
                .await?
                .join()
                .await?;
            assert!(job.is_finished().await?);
            let job = job.bunshin().await;
            assert!(!job.is_finished().await?);

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn sequential() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let job: Job<_> = modes::Sequential::try_fixture()?.into();
            job.start(Ctx::fixture())
                .await?
                .cancel()
                .await?
                .join()
                .await?;
            assert!(job.is_finished().await?);
            let job = job.bunshin().await;
            assert!(!job.is_finished().await?);

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn shell() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let job: Job<_> = modes::Shell::fixture().into();
            job.start(Ctx::fixture())
                .await?
                .cancel()
                .await?
                .join()
                .await?;
            assert!(job.is_finished().await?);
            let job = job.bunshin().await;
            assert!(!job.is_finished().await?);

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn watch() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let job: Job<_> = modes::Watch::try_fixture()?.into();
            job.start(Ctx::fixture())
                .await?
                .cancel()
                .await?
                .join()
                .await?;
            assert!(job.is_finished().await?);
            let job = job.bunshin().await;
            assert!(!job.is_finished().await?);

            Ok(())
        },
    )
}
