use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::Ordering, Arc};

use crate::error::CmdResult;
use crate::task::Runner;
use crate::task::Task;

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
    async fn run(&self) -> CmdResult<Self> {
        self.is_running.store(true, Ordering::Relaxed);
        tokio::spawn({
            let each_sleep_time = self.each_sleep_time;
            let sleep_count = self.sleep_count;
            let is_running = self.is_running.clone();
            let is_finished = self.is_finished.clone();
            let is_cancelled = self.is_cancelled.clone();
            async move {
                for _ in 0..sleep_count {
                    tokio::time::sleep(tokio::time::Duration::from_secs(each_sleep_time)).await;
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

    async fn is_finished(&self) -> CmdResult<bool> {
        Ok(self.is_finished.load(Ordering::Relaxed))
    }

    async fn cancel(&self) -> CmdResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self::new(self.each_sleep_time, self.sleep_count)
    }
}

impl From<Mock> for Task {
    fn from(value: Mock) -> Self {
        Task::Mock(value)
    }
}

mod test {
    use super::*;
    use std::sync::atomic::Ordering;

    #[tokio::test]
    async fn new() {
        let mock = Mock::new(1, 3);

        mock.assert_status(MockStatus {
            is_running: false,
            is_finished: false,
            is_cancelled: false,
        });
    }

    #[tokio::test]
    async fn run_wait() {
        let mock = Mock::new(1, 3);
        let id = mock.id();

        assert!(mock.run().await.is_ok());
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
        let mock = Mock::new(1, 3);
        let id = mock.id();

        assert!(mock.run().await.is_ok());

        assert!(mock.cancel().await.is_ok());
        assert!(mock.is_running.load(Ordering::Relaxed));
        assert!(!mock.is_finished.load(Ordering::Relaxed));
        assert!(mock.is_cancelled.load(Ordering::Relaxed));

        assert!(mock.wait().await.is_ok());
        assert!(!mock.is_running.load(Ordering::Relaxed));

        assert_eq!(mock.id(), id);
    }
}
