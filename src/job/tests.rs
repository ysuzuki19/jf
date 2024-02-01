use crate::util::testutil::*;

use super::*;

impl Job {
    #[cfg_attr(coverage, coverage(off))]
    pub fn as_mock(&self) -> &modes::Mock {
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
            let job: Job = modes::Command::async_fixture().await.into();
            job.start().await?.cancel().await?.join().await?;
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
            let job: Job = modes::Parallel::try_async_fixture().await?.into();
            job.start().await?.cancel().await?.join().await?;
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
            let job: Job = modes::Sequential::try_async_fixture().await?.into();
            job.start().await?.cancel().await?.join().await?;
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
            let job: Job = modes::Shell::async_fixture().await.into();
            job.start().await?.cancel().await?.join().await?;
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
            let job: Job = modes::Watch::try_async_fixture().await?.into();
            job.start().await?.cancel().await?.join().await?;
            assert!(job.is_finished().await?);
            let job = job.bunshin().await;
            assert!(!job.is_finished().await?);

            Ok(())
        },
    )
}
