use crate::job::runner;
use crate::testutil::*;

use super::*;

impl TryFixture for Sequential<MockLogWriter> {
    #[cfg_attr(coverage, coverage(off))]
    fn try_fixture() -> JfResult<Self> {
        let params = SequentialParams {
            jobs: vec!["fast".into(), "fast".into()],
        };
        Sequential::new(params, TryFixture::try_fixture()?)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_empty_job() -> JfResult<()> {
    let params = SequentialParams { jobs: vec![] };
    let must_faile = Sequential::<MockLogWriter>::new(params, TryFixture::try_fixture()?);
    assert!(must_faile.is_err());
    Ok(())
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_unknown_job() -> JfResult<()> {
    let params = SequentialParams {
        jobs: vec!["unknown".into()],
    };
    let must_fail = Sequential::<MockLogWriter>::new(params, TryFixture::try_fixture()?);
    assert!(must_fail.is_err());
    Ok(())
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn new() -> JfResult<()> {
    Sequential::try_fixture()?;
    Ok(())
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn start() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let s = Sequential::try_fixture()?.start(Fixture::fixture()).await?;
            assert!(!s.is_finished().await?);
            for (index, job) in s.jobs.iter().enumerate() {
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
            let s = Sequential::try_fixture()?.start(Fixture::fixture()).await?;
            s.cancel().await?;
            runner::sleep().await; // sleep for job interval
            assert!(s.is_cancelled.load(Ordering::Relaxed));
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn wait() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let s = Sequential::try_fixture()?;
            s.start(Fixture::fixture()).await?.wait().await?;
            assert!(s.is_finished().await?);
            s.jobs.into_iter().for_each(|job| {
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
            let origin = Sequential::try_fixture()?;
            origin.start(Fixture::fixture()).await?.cancel().await?;

            let bunshin = origin.bunshin();
            assert_eq!(origin.jobs.len(), bunshin.jobs.len());
            for (bunshin_job, origin_job) in bunshin.jobs.iter().zip(origin.jobs) {
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
            let s = Sequential::try_fixture()?;
            assert!(!s.is_finished().await?);
            Ok(())
        },
    )
}
