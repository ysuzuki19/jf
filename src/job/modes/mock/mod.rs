mod test;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::Ordering, Arc};

use tokio::sync::Mutex;

use crate::error::JfResult;
use crate::job::Job;
use crate::job::Runner;
use crate::testutil::Fixture;

static MOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, serde::Deserialize)]
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
    handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
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
        let handle = tokio::spawn({
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
        self.handle.lock().await.replace(handle);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        Ok(self.is_finished.load(Ordering::Relaxed))
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.lock().await.take() {
            handle.abort();
        }
        self.is_finished.store(true, Ordering::Relaxed);
        self.is_running.store(false, Ordering::Relaxed);
        Ok(self.clone())
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
    fn fixture() -> Self {
        Self {
            each_sleep_time: 1,
            sleep_count: 3,
        }
    }
}