// SPDX-License-Identifier: MPL-2.0
use crate::util::testutil::*;

use super::*;

const MOCK_SLEEP_TIME: u64 = 1;
const MOCK_SLEEP_COUNT: u8 = 3;

impl Fixture for Mock {
    #[cfg_attr(coverage, coverage(off))]
    fn fixture() -> Self {
        Self::new(MockParams {
            each_sleep_time: MOCK_SLEEP_TIME,
            sleep_count: MOCK_SLEEP_COUNT,
        })
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn new() {
    let mock = Mock::fixture();

    mock.assert_is_started_eq(false)
        .assert_is_running_eq(false)
        .assert_is_finished_eq(false)
        .assert_is_cancelled_eq(false);
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_join() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let mock = Mock::fixture();
            let id = mock.id();

            mock.start()
                .await?
                .assert_is_started_eq(true)
                .assert_is_running_eq(true)
                .assert_is_cancelled_eq(false);
            assert!(mock.join().await?.is_succeed());
            mock.assert_id_eq(id) // not changed mock instance
                .assert_is_started_eq(true)
                .assert_is_running_eq(false)
                .assert_is_finished_eq(true)
                .assert_is_cancelled_eq(false);

            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_cancel_join() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let mock = Mock::fixture();
            let id = mock.id();

            mock.start()
                .await?
                .cancel()
                .await?
                .assert_is_cancelled_eq(true);
            assert!(mock.join().await?.is_failed());
            mock.assert_id_eq(id)
                .assert_is_running_eq(false)
                .assert_is_finished_eq(true);

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
            let origin = Mock::fixture();

            origin.start().await?.cancel().await?;
            origin
                .assert_is_started_eq(true)
                .assert_is_running_eq(false)
                .assert_is_finished_eq(true)
                .assert_is_cancelled_eq(true);

            let bunshin = origin.bunshin().await;
            bunshin
                .assert_id_ne(origin.id) // check new mock job creation
                .assert_each_sleep_time_eq(origin.each_sleep_time)
                .assert_sleep_count_eq(origin.sleep_count)
                .assert_is_started_eq(false)
                .assert_is_running_eq(false)
                .assert_is_finished_eq(false)
                .assert_is_cancelled_eq(false);
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn into_job() {
    let mock = Mock::fixture();
    let id = mock.id();

    if let Job::Mock(mock) = mock.into() {
        mock.assert_id_eq(id)
            .assert_each_sleep_time_eq(MOCK_SLEEP_TIME)
            .assert_sleep_count_eq(MOCK_SLEEP_COUNT)
            .assert_is_started_eq(false)
            .assert_is_running_eq(false)
            .assert_is_finished_eq(false)
            .assert_is_cancelled_eq(false);
    } else {
        panic!("Invalid Variant: Job::Mock expected");
    }
}
