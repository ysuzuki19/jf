use std::io::Write;

use crate::job::runner;
use crate::testutil::*;

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

impl TryFixture for Watch {
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
            assert!(Watch::new(params, TryFixture::try_fixture()?).is_err());
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
            w.start().await?;
            assert!(!w.is_finished().await?);
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
            w.start().await?;
            assert!(!w.is_finished().await?);
            let id = w.running_job.lock().await.as_mock().id();
            std::fs::File::create("./tests/dummy_entities/file1.txt")?.write_all(b"")?;
            runner::sleep().await;
            let id2 = w.running_job.lock().await.as_mock().id();
            assert_ne!(id, id2);
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
            w.start().await?.cancel().await?;
            runner::sleep().await; // for cover breaking loop
            w.wait().await?;
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
            let bunshin = origin.bunshin();
            assert_ne!(origin.job.as_mock().id(), bunshin.job.as_mock().id());
            Ok(())
        },
    )
}
