use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    error::JfResult,
    job::{Job, Runner},
    jobdef::{Agent, JobdefPool},
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WatchParams {
    pub job: String,
    pub watch_list: Vec<String>,
}

#[derive(Clone)]
pub struct Watch {
    job: Box<Job>,
    watch_list: Vec<String>,
    is_cancelled: Arc<AtomicBool>,
}

impl Watch {
    pub fn new(params: WatchParams, pool: JobdefPool) -> JfResult<Self> {
        let job = pool.build(params.job, Agent::Job)?;
        Ok(Self {
            job: Box::new(job),
            watch_list: params.watch_list,
            is_cancelled: Arc::new(AtomicBool::new(false)),
        })
    }
}

#[async_trait::async_trait]
impl Runner for Watch {
    async fn start(&self) -> JfResult<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in self.clone().watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        loop {
            let running_job = self.job.bunshin().start().await?;

            loop {
                match rx.recv()??.kind {
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                        break;
                    }
                    _ => {}
                }
            }

            running_job.cancel().await?;
            if self.is_cancelled.load(Ordering::Relaxed) {
                break;
            }
        }
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        Ok(false)
    }

    async fn cancel(&self) -> JfResult<()> {
        self.is_cancelled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn bunshin(&self) -> Self {
        Self {
            job: Box::new(self.job.bunshin()),
            watch_list: self.watch_list.clone(),
            is_cancelled: Arc::new(AtomicBool::new(false)),
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
    #[coverage(off)]
    pub fn watch_list() -> Vec<String> {
        vec!["./tests/dummy_entities/*".to_string()]
    }
}

#[cfg(test)]
mod test {
    use crate::testutil::{Fixture, TryFixture};

    use super::*;

    impl Fixture for WatchParams {
        #[coverage(off)]
        fn gen() -> Self {
            WatchParams {
                job: "fast".to_string(),
                watch_list: fixtures::watch_list(),
            }
        }
    }

    impl TryFixture for Watch {
        #[coverage(off)]
        fn try_gen() -> JfResult<Self> {
            Watch::new(Fixture::gen(), TryFixture::try_gen()?)
        }
    }

    #[tokio::test]
    #[coverage(off)]
    async fn invalid_new_with_unknown_job() -> JfResult<()> {
        let params = WatchParams {
            job: "unknown".to_string(),
            watch_list: fixtures::watch_list(),
        };
        assert!(Watch::new(params, TryFixture::try_gen()?).is_err());
        Ok(())
    }

    #[tokio::test]
    #[coverage(off)]
    async fn new() -> JfResult<()> {
        let w = Watch::try_gen()?;
        assert!(!w.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    #[coverage(off)]
    async fn bunshin() -> JfResult<()> {
        let origin = Watch::try_gen()?;
        let bunshin = origin.bunshin();
        assert_ne!(origin.job.as_mock().id(), bunshin.job.as_mock().id());
        Ok(())
    }
}
