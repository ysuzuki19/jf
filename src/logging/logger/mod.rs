mod log_level;

use tokio::sync::mpsc;

use crate::util::error::JfResult;

pub use self::log_level::LogLevel;

use super::Message;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Logger {
    tx: mpsc::Sender<Message>,
    log_level: LogLevel,
}

#[cfg(test)]
impl PartialEq for Logger {
    fn eq(&self, other: &Self) -> bool {
        self.log_level == other.log_level
    }
}

impl Logger {
    pub fn new(tx: mpsc::Sender<Message>, log_level: LogLevel) -> Self {
        Self { tx, log_level }
    }

    #[cfg(test)]
    pub fn level(&self) -> LogLevel {
        self.log_level
    }

    #[cfg(test)]
    pub fn update(&mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
        self.clone()
    }

    async fn send(&mut self, msg: Message) -> JfResult<()> {
        self.tx.send(msg).await?;
        Ok(())
    }

    async fn send_with_guard(&mut self, log_level: LogLevel, msg: Message) -> JfResult<()> {
        if self.log_level >= log_level {
            self.send(msg).await?;
        }
        Ok(())
    }

    pub async fn force<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send(Message {
            line: line.as_ref().to_string(),
        })
        .await?;
        Ok(())
    }

    pub async fn error<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Error,
            Message {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn warn<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Warn,
            Message {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }

    pub async fn info<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Info,
            Message {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }

    pub async fn debug<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(
            LogLevel::Debug,
            Message {
                line: line.as_ref().to_string(),
            },
        )
        .await?;
        Ok(())
    }
}
