use crate::{
    job::Runner,
    jobdef::JobdefPool,
    util::{error::JfResult, testutil::*},
};

use super::*;

impl TryAsyncFixture for Parallel {
    #[cfg_attr(coverage, coverage(off))]
    async fn try_async_fixture() -> JfResult<Self> {
        let params = ParallelParams {
            jobs: vec!["fast".into(), "fast".into()],
        };
        Parallel::new(
            Ctx::async_fixture().await,
            params,
            TryFixture::try_fixture()?,
        )
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_unknown_job() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let must_fail = Parallel::new(
                Ctx::async_fixture().await,
                ParallelParams {
                    jobs: vec!["mock".into(), "mock".into()],
                },
                JobdefPool::new(vec![]),
            );
            assert!(must_fail.is_err());
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn new() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_async_fixture().await?;
            assert_eq!(p.jobs.len(), 2);

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn start() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_async_fixture().await?;
            p.start().await?;
            for job in p.jobs {
                job.as_mock().assert_is_started_eq(true);
            }
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn cancel() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_async_fixture().await?;
            p.start().await?.cancel().await?;
            for job in p.jobs {
                job.as_mock()
                    .assert_is_started_eq(true)
                    .assert_is_cancelled_eq(true);
            }
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn join() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_async_fixture().await?;
            p.start().await?;
            assert!(p.join().await?.is_succeed());
            p.jobs.into_iter().for_each(|job| {
                job.as_mock()
                    .assert_is_started_eq(true)
                    .assert_is_finished_eq(true);
            });
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn bunshin() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let origin = Parallel::try_async_fixture().await?;
            origin.start().await?.cancel().await?;

            let bunshin = origin.bunshin().await;
            assert_eq!(origin.jobs.len(), bunshin.jobs.len());
            for (bunshin_job, origin_job) in bunshin.jobs.iter().zip(origin.jobs) {
                bunshin_job
                    .as_mock()
                    .assert_id_ne(origin_job.as_mock().id())
                    .assert_is_started_eq(false);
            }
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn is_finished_not_yet_started() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_async_fixture().await?;
            assert!(!p.is_finished().await?);
            Ok(())
        },
    )
}
