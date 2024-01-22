use crate::testutil::*;
use crate::{error::JfResult, job::Runner, jobdef::JobdefPool};

use super::*;

impl Fixture for ParallelParams {
    #[cfg_attr(coverage, coverage(off))]
    fn fixture() -> Self {
        Self {
            jobs: vec!["fast".into(), "fast".into()],
        }
    }
}

impl TryFixture for Parallel {
    #[cfg_attr(coverage, coverage(off))]
    fn try_fixture() -> JfResult<Self> {
        Parallel::new(Fixture::fixture(), TryFixture::try_fixture()?)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_unknown_job() -> JfResult<()> {
    let must_fail = Parallel::new(
        ParallelParams {
            jobs: vec!["mock".into(), "mock".into()],
        },
        JobdefPool::new(vec![]),
    );
    assert!(must_fail.is_err());
    Ok(())
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn new() -> JfResult<()> {
    let p = Parallel::try_fixture()?;
    assert!(p.jobs.len() == 2);
    Ok(())
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn start() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_fixture()?;
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
            let p = Parallel::try_fixture()?;
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
fn wait() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let p = Parallel::try_fixture()?;
            p.start()
                .await?
                .wait()
                .await?
                .jobs
                .into_iter()
                .for_each(|job| {
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
            let origin = Parallel::try_fixture()?;
            origin.start().await?.cancel().await?;

            let bunshin = origin.bunshin();
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
            let p = Parallel::try_fixture()?;
            assert!(!p.is_finished().await?);
            Ok(())
        },
    )
}
