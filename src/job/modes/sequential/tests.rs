use crate::job::runner;
use crate::util::testutil::*;

use super::*;

impl TryAsyncFixture for Sequential {
    #[cfg_attr(coverage, coverage(off))]
    async fn try_async_fixture() -> JfResult<Self> {
        let params = SequentialParams {
            jobs: vec!["fast".into(), "fast".into()],
        };
        Sequential::new(
            Ctx::async_fixture().await,
            params,
            TryFixture::try_fixture()?,
        )
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_empty_job() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let params = SequentialParams { jobs: vec![] };
            let must_faile = Sequential::new(
                Ctx::async_fixture().await,
                params,
                TryFixture::try_fixture()?,
            );
            assert!(must_faile.is_err());

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_unknown_job() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let params = SequentialParams {
                jobs: vec!["unknown".into()],
            };
            let must_fail = Sequential::new(
                Ctx::async_fixture().await,
                params,
                TryFixture::try_fixture()?,
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
            Sequential::try_async_fixture().await?;
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
            let s = Sequential::try_async_fixture().await?.start().await?;
            assert!(!s.is_finished().await?);
            for (index, job) in s.jobs.read().iter().enumerate() {
                if index == 0 {
                    // first job is started immediately
                    job.as_mock().assert_is_started_eq(true);
                } else {
                    // other jobs are not started yet
                    job.as_mock().assert_is_started_eq(false);
                }
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
            let s = Sequential::try_async_fixture().await?;
            s.cancel().await?;
            runner::interval().await; // sleep for job interval
            assert!(s.canceller.is_canceled());
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
            let s = Sequential::try_async_fixture().await?;
            s.start().await?;
            assert!(s.join().await?.is_succeed());
            assert!(s.is_finished().await?);
            s.jobs.read().iter().for_each(|job| {
                job.as_mock().assert_is_finished_eq(true);
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
            let origin = Sequential::try_async_fixture().await?;
            origin.start().await?.cancel().await?;

            let bunshin = origin.bunshin().await;
            assert_eq!(origin.jobs.read().len(), bunshin.jobs.read().len());
            for (bunshin_job, origin_job) in bunshin.jobs.read().iter().zip(origin.jobs.read()) {
                bunshin_job
                    .as_mock()
                    .assert_id_ne(origin_job.as_mock().id())
                    .assert_is_started_eq(false)
                    .assert_is_cancelled_eq(false);
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
            let s = Sequential::try_async_fixture().await?;
            assert!(!s.is_finished().await?);
            Ok(())
        },
    )
}
