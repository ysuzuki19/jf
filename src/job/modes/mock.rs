use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::Ordering, Arc};

use crate::error::JfResult;
use crate::job::Job;
use crate::job::Runner;
use crate::testutil::Fixture;

static MOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MockParams {
    pub each_sleep_time: u64,
    pub sleep_count: u8,
}

#[derive(Clone, Default)]
pub struct Mock {
    each_sleep_time: u64,
    sleep_count: u8,
    id: usize,
    is_started: Arc<AtomicBool>,
    is_running: Arc<AtomicBool>,
    is_finished: Arc<AtomicBool>,
    is_cancelled: Arc<AtomicBool>,
}

impl Mock {
    pub fn new(params: MockParams) -> Self {
        Self {
            each_sleep_time: params.each_sleep_time,
            sleep_count: params.sleep_count,
            id: MOCK_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            ..Default::default()
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn assert_id_eq(&self, id: usize) -> Self {
        assert_eq!(
            self.id, id,
            "Mock({}).id is expected {} but {}",
            self.id, id, self.id
        );
        self.clone()
    }

    pub fn assert_id_ne(&self, id: usize) -> Self {
        assert_ne!(self.id, id, "Mock({}).id is expected not {}", self.id, id);
        self.clone()
    }

    pub fn assert_each_sleep_time_eq(&self, sleep_time: u64) -> Self {
        assert_eq!(
            self.each_sleep_time, sleep_time,
            "Mock({}).each_sleep_time is expected {} but {}",
            self.id, sleep_time, self.each_sleep_time
        );
        self.clone()
    }

    pub fn assert_sleep_count_eq(&self, sleep_count: u8) -> Self {
        assert_eq!(
            self.sleep_count, sleep_count,
            "Mock({}).sleep_count is expected {} but {}",
            self.id, sleep_count, self.sleep_count
        );
        self.clone()
    }

    pub fn assert_is_started_eq(&self, is_started: bool) -> Self {
        assert_eq!(
            self.is_started.load(Ordering::Relaxed),
            is_started,
            "Mock({}).is_started is expected {} but {}",
            self.id,
            is_started,
            self.is_started.load(Ordering::Relaxed)
        );
        self.clone()
    }

    pub fn assert_is_running_eq(&self, is_running: bool) -> Self {
        assert_eq!(
            self.is_running.load(Ordering::Relaxed),
            is_running,
            "Mock({}).is_running is expected {} but {}",
            self.id,
            is_running,
            self.is_running.load(Ordering::Relaxed)
        );
        self.clone()
    }

    pub fn assert_is_finished_eq(&self, is_finished: bool) -> Self {
        assert_eq!(
            self.is_finished.load(Ordering::Relaxed),
            is_finished,
            "Mock({}).is_finished is expected {} but {}",
            self.id,
            is_finished,
            self.is_finished.load(Ordering::Relaxed)
        );
        self.clone()
    }

    pub fn assert_is_cancelled_eq(&self, is_cancelled: bool) -> Self {
        assert_eq!(
            self.is_cancelled.load(Ordering::Relaxed),
            is_cancelled,
            "Mock({}).is_cancelled is expected {} but {}",
            self.id,
            is_cancelled,
            self.is_cancelled.load(Ordering::Relaxed)
        );
        self.clone()
    }
}

#[async_trait::async_trait]
impl Runner for Mock {
    async fn start(&self) -> JfResult<Self> {
        self.is_started.store(true, Ordering::Relaxed);
        self.is_running.store(true, Ordering::Relaxed);
        tokio::spawn({
            let each_sleep_time = self.each_sleep_time;
            let sleep_count = self.sleep_count;
            let is_running = self.is_running.clone();
            let is_finished = self.is_finished.clone();
            let is_cancelled = self.is_cancelled.clone();
            async move {
                for _ in 0..sleep_count {
                    tokio::time::sleep(tokio::time::Duration::from_millis(each_sleep_time)).await;
                    if is_cancelled.load(Ordering::Relaxed) {
                        break;
                    }
                }
                is_running.store(false, Ordering::Relaxed);
                is_finished.store(true, Ordering::Relaxed);
            }
        });
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        Ok(self.is_finished.load(Ordering::Relaxed))
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self::new(MockParams {
            each_sleep_time: self.each_sleep_time,
            sleep_count: self.sleep_count,
        })
    }
}

impl From<Mock> for Job {
    fn from(value: Mock) -> Self {
        Self::Mock(value)
    }
}

impl Fixture for MockParams {
    #[cfg_attr(coverage, coverage(off))]
    fn gen() -> Self {
        Self {
            each_sleep_time: 1,
            sleep_count: 3,
        }
    }
}

mod test {
    use crate::testutil::async_test;

    use super::*;

    const MOCK_SLEEP_TIME: u64 = 1;
    const MOCK_SLEEP_COUNT: u8 = 3;

    #[cfg_attr(coverage, coverage(off))]
    fn test_mock_factory() -> Mock {
        Mock::new(MockParams {
            each_sleep_time: MOCK_SLEEP_TIME,
            sleep_count: MOCK_SLEEP_COUNT,
        })
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn new() {
        let mock = test_mock_factory();

        mock.assert_is_started_eq(false)
            .assert_is_running_eq(false)
            .assert_is_finished_eq(false)
            .assert_is_cancelled_eq(false);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn run_wait() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mock = test_mock_factory();
                let id = mock.id();

                mock.start().await?;
                mock.assert_is_started_eq(true)
                    .assert_is_running_eq(true)
                    .assert_is_cancelled_eq(false);

                mock.wait().await?;
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
    fn run_cancel_wait() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mock = test_mock_factory();
                let id = mock.id();

                mock.start().await?.cancel().await?;
                mock.assert_is_cancelled_eq(true);

                mock.wait().await?;
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
                let origin = test_mock_factory();

                origin.start().await?.cancel().await?;
                origin
                    .assert_is_started_eq(true)
                    .assert_is_running_eq(true)
                    .assert_is_finished_eq(false)
                    .assert_is_cancelled_eq(true);

                let bunshin = origin.bunshin();
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
        let mock = test_mock_factory();
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
}
