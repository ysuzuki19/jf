// SPDX-License-Identifier: MPL-2.0
use std::io::Write;

// use crate::job::runner;
use crate::util::testutil::*;

use super::*;

#[cfg(test)]
mod fixtures {
    #[coverage(off)]
    pub fn watch_list() -> Vec<String> {
        vec!["./tests/dummy_entities/*".to_string()]
    }
}

impl Fixture for WatchParams {
    #[coverage(off)]
    fn fixture() -> Self {
        WatchParams {
            job: "fast".to_string(),
            watch_list: fixtures::watch_list(),
        }
    }
}

impl TryAsyncFixture for Watch {
    #[coverage(off)]
    async fn try_async_fixture() -> JfResult<Self> {
        Watch::new(
            Ctx::async_fixture().await,
            Fixture::fixture(),
            TryFixture::try_fixture()?,
        )
    }
}

#[test]
#[coverage(off)]
fn invalid_new_with_unknown_job() -> JfResult<()> {
    async_test(
        #[coverage(off)]
        async {
            let params = WatchParams {
                job: "unknown".to_string(),
                watch_list: fixtures::watch_list(),
            };
            assert!(Watch::new(
                Ctx::async_fixture().await,
                params,
                TryFixture::try_fixture()?
            )
            .is_err());
            Ok(())
        },
    )
}

#[test]
#[coverage(off)]
fn new() -> JfResult<()> {
    async_test(
        #[coverage(off)]
        async {
            let w = Watch::try_async_fixture().await?;
            assert!(!w.is_finished().await?);
            Ok(())
        },
    )
}

#[test]
#[coverage(off)]
fn start_cancel() -> JfResult<()> {
    async_test(
        #[coverage(off)]
        async {
            let w = Watch::try_async_fixture().await?;
            w.start().await?.cancel().await?;
            // runner::interval().await; // for cover breaking loop
            assert!(w.join().await?.is_failed());
            assert!(w.is_finished().await?);
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
            let w = Watch::try_async_fixture().await?;
            w.start().await?;
            assert!(!w.is_finished().await?);
            let id = w.job.lock().await.as_mock().id();
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            std::fs::File::create("./tests/dummy_entities/file1.txt")?.write_all(b"")?;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            let id2 = w.job.lock().await.as_mock().id();
            assert_ne!(id, id2);
            w.cancel().await?;
            Ok(())
        },
    )
}

#[test]
#[coverage(off)]
fn bunshin() -> JfResult<()> {
    async_test(
        #[coverage(off)]
        async {
            let origin = Watch::try_async_fixture().await?;
            let bunshin = origin.bunshin().await;
            assert_ne!(
                origin.job.lock().await.as_mock().id(),
                bunshin.job.lock().await.as_mock().id()
            );
            Ok(())
        },
    )
}
