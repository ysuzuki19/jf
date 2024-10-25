// SPDX-License-Identifier: MPL-2.0
use crate::util::testutil::*;

use super::*;

impl Job {
    #[coverage(off)]
    pub fn as_mock(&self) -> &modes::Mock {
        if let Self::Mock(t) = self {
            t
        } else {
            panic!("invalid job type expected Mock");
        }
    }
}

#[test]
#[coverage(off)]
fn command() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn parallel() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn sequential() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn shell() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn watch() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
