use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::Ordering, Arc};

use crate::error::JfResult;
use crate::job::Job;
use crate::job::Runner;

static MOCK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct CheckList {
    is_started: Option<bool>,
    is_running: Option<bool>,
    is_finished: Option<bool>,
    is_cancelled: Option<bool>,
}

impl CheckList {
    pub fn new() -> Self {
        Self {
            is_started: None,
            is_running: None,
            is_finished: None,
            is_cancelled: None,
        }
    }

    pub fn started(mut self, flag: bool) -> Self {
        self.is_started = Some(flag);
        self
    }

    pub fn running(mut self, flag: bool) -> Self {
        self.is_running = Some(flag);
        self
    }

    pub fn finished(mut self, flag: bool) -> Self {
        self.is_finished = Some(flag);
        self
    }

    pub fn cancelled(mut self, flag: bool) -> Self {
        self.is_cancelled = Some(flag);
        self
    }
}

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

    pub fn assert_id_eq(&self, id: usize) {
        assert_eq!(
            self.id, id,
            "Mock({}).id is expected {} but {}",
            self.id, id, self.id
        );
    }

    pub fn assert_id_ne(&self, id: usize) {
        assert_ne!(self.id, id, "Mock({}).id is expected not {}", self.id, id);
    }

    pub fn assert_each_sleep_time_eq(&self, sleep_time: u64) {
        assert_eq!(
            self.each_sleep_time, sleep_time,
            "Mock({}).each_sleep_time is expected {} but {}",
            self.id, sleep_time, self.each_sleep_time
        );
    }

    pub fn assert_sleep_count_eq(&self, sleep_count: u8) {
        assert_eq!(
            self.sleep_count, sleep_count,
            "Mock({}).sleep_count is expected {} but {}",
            self.id, sleep_count, self.sleep_count
        );
    }

    fn assert_status(&self, check_list: CheckList) {
        if let Some(is_started) = check_list.is_started {
            let flag = self.is_started.load(Ordering::Relaxed);
            assert_eq!(
                flag, is_started,
                "Mock({}).is_started is expected {} but {}",
                self.id, is_started, flag
            );
        }
        if let Some(is_running) = check_list.is_running {
            let flag = self.is_running.load(Ordering::Relaxed);
            assert_eq!(
                flag, is_running,
                "Mock({}).is_running is expected {} but {}",
                self.id, is_running, flag
            );
        }
        if let Some(is_finished) = check_list.is_finished {
            let flag = self.is_finished.load(Ordering::Relaxed);
            assert_eq!(
                flag, is_finished,
                "Mock({}).is_finished is expected {} but {}",
                self.id, is_finished, flag
            );
        }
        if let Some(is_cancelled) = check_list.is_cancelled {
            let flag = self.is_cancelled.load(Ordering::Relaxed);
            assert_eq!(
                flag, is_cancelled,
                "Mock({}).is_cancelled is expected {} but {}",
                self.id, is_cancelled, flag
            );
        }
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
        Job::Mock(value)
    }
}

mod test {
    use super::*;

    const MOCK_SLEEP_TIME: u64 = 1;
    const MOCK_SLEEP_COUNT: u8 = 3;

    fn test_mock_factory() -> Mock {
        Mock::new(MockParams {
            each_sleep_time: MOCK_SLEEP_TIME,
            sleep_count: MOCK_SLEEP_COUNT,
        })
    }

    #[tokio::test]
    async fn new() {
        let mock = test_mock_factory();

        mock.assert_status(
            CheckList::new()
                .started(false)
                .running(false)
                .finished(false)
                .cancelled(false),
        );
    }

    #[tokio::test]
    async fn run_wait() -> JfResult<()> {
        let mock = test_mock_factory();
        let id = mock.id();

        mock.start().await?;
        mock.assert_status(
            CheckList::new()
                .started(true)
                .running(true)
                .cancelled(false),
        );

        mock.wait().await?;
        mock.assert_status(
            CheckList::new()
                .started(true)
                .running(false)
                .finished(true)
                .cancelled(false),
        );

        mock.assert_id_eq(id);
        Ok(())
    }

    #[tokio::test]
    async fn run_cancel_wait() -> JfResult<()> {
        let mock = test_mock_factory();
        let id = mock.id();

        mock.start().await?.cancel().await?;
        mock.assert_status(CheckList::new().cancelled(true));

        mock.wait().await?;
        mock.assert_status(CheckList::new().running(false).finished(true));

        mock.assert_id_eq(id);

        Ok(())
    }

    #[tokio::test]
    async fn bunshin() -> JfResult<()> {
        let mock = test_mock_factory();
        let id = mock.id();

        mock.start().await?.cancel().await?;
        mock.assert_status(
            CheckList::new()
                .started(true)
                .running(true)
                .finished(false)
                .cancelled(true),
        );

        let bunshin = mock.bunshin();
        bunshin.assert_id_ne(id); // check new mock job creation
        bunshin.assert_each_sleep_time_eq(mock.each_sleep_time);
        bunshin.assert_sleep_count_eq(mock.sleep_count);
        bunshin.assert_status(
            CheckList::new()
                .started(false)
                .running(false)
                .finished(false)
                .cancelled(false),
        );
        Ok(())
    }

    #[tokio::test]
    async fn from() {
        let mock = test_mock_factory();
        let id = mock.id();

        if let Job::Mock(mock) = mock.into() {
            mock.assert_id_eq(id);
            mock.assert_each_sleep_time_eq(MOCK_SLEEP_TIME);
            mock.assert_sleep_count_eq(MOCK_SLEEP_COUNT);
            mock.assert_status(
                CheckList::new()
                    .started(false)
                    .running(false)
                    .finished(false)
                    .cancelled(false),
            );
        } else {
            panic!("Invalid Variant: Job::Mock expected");
        }
    }
}
