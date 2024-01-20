use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::error::JfResult;

type NotifyPayload = Result<notify::Event, notify::Error>;

pub struct WatcherContainer {
    _rw: RecommendedWatcher,
}

impl WatcherContainer {
    pub fn new(
        watch_list: &Vec<String>,
    ) -> JfResult<(Self, std::sync::mpsc::Receiver<NotifyPayload>)> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        for watch_item in watch_list {
            for entry in glob::glob(watch_item.as_str())? {
                watcher.watch(entry?.as_path(), RecursiveMode::NonRecursive)?;
            }
        }

        Ok((Self { _rw: watcher }, rx))
    }
}
