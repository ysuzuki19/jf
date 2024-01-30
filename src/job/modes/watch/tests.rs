use std::io::Write;

use crate::ctx::logger::MockLogWriter;
use crate::job::runner;
use crate::util::testutil::*;

use super::*;

#[cfg(test)]
mod fixtures {
    #[cfg_attr(coverage, coverage(off))]
    pub fn watch_list() -> Vec<String> {
        vec!["./tests/dummy_entities/*".to_string()]
    }
}

impl Fixture for WatchParams {
    #[cfg_attr(coverage, coverage(off))]
    fn fixture() -> Self {
        WatchParams {
            job: "fast".to_string(),
            watch_list: fixtures::watch_list(),
        }
    }
}

impl TryFixture for Watch<MockLogWriter> {
    #[cfg_attr(coverage, coverage(off))]
    fn try_fixture() -> JfResult<Self> {
        Watch::new(Fixture::fixture(), TryFixture::try_fixture()?)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn invalid_new_with_unknown_job() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let params = WatchParams {
                job: "unknown".to_string(),
                watch_list: fixtures::watch_list(),
            };
            assert!(Watch::<MockLogWriter>::new(params, TryFixture::try_fixture()?).is_err());
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
            let w = Watch::try_fixture()?;
            assert!(!w.is_finished().await?);
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
            let w = Watch::try_fixture()?;
            w.start(Fixture::fixture()).await?;
            assert!(!w.is_finished().await?);
            w.cancel().await?.join().await?;
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
            let w = Watch::try_fixture()?;
            w.start(Fixture::fixture()).await?;
            assert!(!w.is_finished().await?);
            let id = w.running_job.lock().await.clone().unwrap().as_mock().id();
            std::fs::File::create("./tests/dummy_entities/file1.txt")?.write_all(b"")?;
            runner::sleep().await;
            let id2 = w.running_job.lock().await.clone().unwrap().as_mock().id();
            assert_ne!(id, id2);
            w.cancel().await?;
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
            let w = Watch::try_fixture()?;
            w.start(Fixture::fixture()).await?.cancel().await?;
            runner::sleep().await; // for cover breaking loop
            w.join().await?;
            assert!(w.is_finished().await?);
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
            let origin = Watch::try_fixture()?;
            let bunshin = origin.bunshin().await;
            assert_ne!(
                origin.job.read().as_mock().id(),
                bunshin.job.read().as_mock().id()
            );
            Ok(())
        },
    )
}
