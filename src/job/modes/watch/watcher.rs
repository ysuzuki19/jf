use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::util::{
    error::{JfError, JfResult},
    ReadOnly,
};

use super::INTERVAL_MILLIS;

type NotifyPayload = Result<notify::Event, notify::Error>;

pub struct JfWatcher {
    _watcher: ReadOnly<RecommendedWatcher>, // not used but needed to keep the watcher alive
    rx: std::sync::mpsc::Receiver<NotifyPayload>,
    is_cancelled: Arc<AtomicBool>,
}

impl JfWatcher {
    pub fn new(watch_list: &Vec<String>, parent_cancelled: Arc<AtomicBool>) -> JfResult<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        Ok(Self {
            _watcher: watcher.into(),
            rx,
            is_cancelled: parent_cancelled,
        })
    }

    pub async fn wait(self) -> JfResult<()> {
        tokio::task::spawn_blocking(move || {
            loop {
                if self.is_cancelled.load(Ordering::Relaxed) {
                    break;
                }
                match self
                    .rx
                    .recv_timeout(std::time::Duration::from_millis(INTERVAL_MILLIS))
                {
                    Ok(event) => match event?.kind {
                        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                            break;
                        }
                        _ => {}
                    },
                    Err(e) => match e {
                        std::sync::mpsc::RecvTimeoutError::Timeout => {
                            sleep(std::time::Duration::from_millis(INTERVAL_MILLIS));
                            continue;
                        }
                        std::sync::mpsc::RecvTimeoutError::Disconnected => {
                            return Err(JfError::from(e));
                        }
                    },
                }
            }
            Ok(())
        })
        .await?
    }
}
