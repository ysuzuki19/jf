// SPDX-License-Identifier: MPL-2.0
use std::thread::sleep;

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    job::canceller::Canceller,
    util::{
        error::{JfError, JfResult},
        ReadOnly,
    },
};

use super::INTERVAL_MILLIS;

type NotifyPayload = Result<notify::Event, notify::Error>;

pub struct JfWatcher {
    _watcher: ReadOnly<RecommendedWatcher>, // not used but needed to keep the watcher alive
    rx: std::sync::mpsc::Receiver<NotifyPayload>,
    canceller: Canceller,
}

impl JfWatcher {
    pub fn new(watch_list: &Vec<String>, canceller: Canceller) -> JfResult<Self> {
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
            canceller,
        })
    }

    pub async fn wait(self) -> JfResult<()> {
        tokio::task::spawn_blocking(move || {
            loop {
                if self.canceller.is_canceled() {
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
