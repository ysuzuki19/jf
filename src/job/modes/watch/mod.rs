mod watcher_container;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use notify::EventKind;
use tokio::sync::Mutex;

use crate::{
    error::JfResult,
    job::{types::JfHandle, Job, Runner},
    jobdef::{Agent, JobdefPool},
};

use self::watcher_container::WatcherContainer;

#[derive(Clone, serde::Deserialize)]
pub struct WatchParams {
    pub job: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch {
    job: Box<Job>,
    running_job: Arc<Mutex<Job>>,
    watch_list: Vec<String>,
    is_cancelled: Arc<AtomicBool>,
    handle: Arc<Mutex<Option<JfHandle>>>,
    watcher_container: Arc<Mutex<Option<WatcherContainer>>>,
}

impl Watch {
    pub fn new(params: WatchParams, pool: JobdefPool) -> JfResult<Self> {
        let job = pool.build(params.job, Agent::Job)?;
        Ok(Self {
            job: Box::new(job.clone()),
            running_job: Arc::new(Mutex::new(job)),
            watch_list: params.watch_list,
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
            watcher_container: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Watch {
    async fn start(&self) -> JfResult<Self> {
        let (container, rx) = WatcherContainer::new(&self.watch_list)?;
        self.watcher_container.lock().await.replace(container);

        let handle = tokio::spawn({
            let job = self.job.clone();
            let running_job = self.running_job.clone();
            let is_cancelled = self.is_cancelled.clone();

            async move {
                loop {
                    *(running_job.lock().await) = job.bunshin().start().await?;

                    loop {
                        match rx.recv()??.kind {
                            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                                break;
                            }
                            _ => {}
                        }
                    }

                    running_job.lock().await.cancel().await?;
                    if is_cancelled.load(Ordering::Relaxed) {
                        break;
                    }
                }
                Ok(())
            }
        });
        self.handle.lock().await.replace(handle);
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        Ok(self.is_cancelled.load(Ordering::Relaxed))
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        self.running_job.lock().await.cancel().await?;
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            job: Box::new(self.job.bunshin()),
            running_job: Arc::new(Mutex::new(self.job.bunshin())),
            watch_list: self.watch_list.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            handle: Arc::new(Mutex::new(None)),
            watcher_container: Arc::new(Mutex::new(None)),
        }
    }
}

impl From<Watch> for Job {
    fn from(value: Watch) -> Self {
        Self::Watch(value)
    }
}

#[cfg(test)]
mod fixtures {
    #[cfg_attr(coverage, coverage(off))]
    pub fn watch_list() -> Vec<String> {
        vec!["./tests/dummy_entities/*".to_string()]
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use crate::{
        job::runner,
        testutil::{async_test, Fixture, TryFixture},
    };

    use super::*;

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
                assert!(w.start().await.is_ok());
                assert!(w.cancel().await.is_ok());
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
}
