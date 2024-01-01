use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::Ordering, Arc};

use crate::error::JfResult;
use crate::job::Job;
use crate::job::Runner;

static MOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct MockStatus {
    is_running: bool,
    is_finished: bool,
    is_cancelled: bool,
}

#[derive(Clone, Default)]
pub struct Mock {
    each_sleep_time: u64,
    sleep_count: u8,
    id: usize,
    is_running: Arc<AtomicBool>,
    is_finished: Arc<AtomicBool>,
    is_cancelled: Arc<AtomicBool>,
}

impl Mock {
    pub fn new(each_sleep_time: u64, sleep_count: u8) -> Self {
        Self {
            each_sleep_time,
            sleep_count,
            id: MOCK_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            ..Default::default()
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn assert_status(&self, status: MockStatus) {
        assert_eq!(self.is_running.load(Ordering::Relaxed), status.is_running);
        assert_eq!(self.is_finished.load(Ordering::Relaxed), status.is_finished);
        assert_eq!(
            self.is_cancelled.load(Ordering::Relaxed),
            status.is_cancelled
        );
    }
}

#[async_trait::async_trait]
impl Runner for Mock {
    async fn start(&self) -> JfResult<Self> {
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
        Self::new(self.each_sleep_time, self.sleep_count)
    }
}

impl From<Mock> for Job {
    fn from(value: Mock) -> Self {
        Job::Mock(value)
    }
}

mod test {
    use super::*;
    use std::sync::atomic::Ordering;

    const MOCK_SLEEP_TIME: u64 = 1;
    const MOCK_SLEEP_COUNT: u8 = 3;

    fn test_mock_factory() -> Mock {
        Mock::new(MOCK_SLEEP_TIME, MOCK_SLEEP_COUNT)
    }

    #[tokio::test]
    async fn new() {
        let mock = test_mock_factory();

        mock.assert_status(MockStatus {
            is_running: false,
            is_finished: false,
            is_cancelled: false,
        });
    }

    #[tokio::test]
    async fn run_wait() {
        let mock = test_mock_factory();
        let id = mock.id();

        assert!(mock.start().await.is_ok());
        mock.assert_status(MockStatus {
            is_running: true,
            is_finished: false,
            is_cancelled: false,
        });

        assert!(mock.wait().await.is_ok());
        mock.assert_status(MockStatus {
            is_running: false,
            is_finished: true,
            is_cancelled: false,
        });

        assert_eq!(mock.id(), id);
    }

    #[tokio::test]
    async fn run_cancel_wait() {
        let mock = test_mock_factory();
        let id = mock.id();

        assert!(mock.start().await.is_ok());

        assert!(mock.cancel().await.is_ok());
        assert!(mock.is_running.load(Ordering::Relaxed));
        assert!(!mock.is_finished.load(Ordering::Relaxed));
        assert!(mock.is_cancelled.load(Ordering::Relaxed));

        assert!(mock.wait().await.is_ok());
        assert!(!mock.is_running.load(Ordering::Relaxed));

        assert_eq!(mock.id(), id);
    }

    #[tokio::test]
    async fn bunshin() {
        let mock = test_mock_factory();
        let id = mock.id();

        let bunshin = mock.bunshin();
        assert_ne!(bunshin.id(), id);
        assert_eq!(bunshin.each_sleep_time, mock.each_sleep_time);
        assert_eq!(bunshin.sleep_count, mock.sleep_count);
        assert!(!bunshin.is_running.load(Ordering::Relaxed));
        assert!(!bunshin.is_finished.load(Ordering::Relaxed));
        assert!(!bunshin.is_cancelled.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn from() {
        let mock = test_mock_factory();
        let id = mock.id();

        if let Job::Mock(mock) = mock.into() {
            assert_eq!(mock.id(), id);
            assert_eq!(mock.each_sleep_time, MOCK_SLEEP_TIME);
            assert_eq!(mock.sleep_count, MOCK_SLEEP_COUNT);
            assert!(!mock.is_running.load(Ordering::Relaxed));
            assert!(!mock.is_finished.load(Ordering::Relaxed));
            assert!(!mock.is_cancelled.load(Ordering::Relaxed));
        } else {
            panic!("Invalid Variant: Job::Mock expected");
        }
    }
}
