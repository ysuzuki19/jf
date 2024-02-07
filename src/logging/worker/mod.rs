mod writer;

use tokio::sync::mpsc;

use crate::util::error::JfResult;

use super::{logger::Logger, LogLevel};
pub use writer::*;

pub struct Worker {
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl Worker {
    pub fn new() -> Self {
        Self { handle: None }
    }

    pub async fn start<W: Writer>(&mut self, mut log_writer: W, log_level: LogLevel) -> Logger {
        let (tx, mut rx) = mpsc::channel::<String>(100);
        self.handle = Some(tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                log_writer.write(&msg).await.unwrap();
            }
        }));
        Logger::new(tx, log_level)
    }

    pub async fn join(mut self) -> JfResult<()> {
        if let Some(handle) = self.handle.take() {
            handle.await?;
        }
        Ok(())
    }
}
