mod log_generator;
mod log_level;

use tokio::sync::mpsc;

use crate::util::error::JfResult;

pub use self::log_level::LogLevel;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Logger {
    tx: mpsc::Sender<String>,
    log_level: LogLevel,
}

#[cfg(test)]
impl PartialEq for Logger {
    fn eq(&self, other: &Self) -> bool {
        self.log_level == other.log_level
    }
}

impl Logger {
    pub fn new(tx: mpsc::Sender<String>, log_level: LogLevel) -> Self {
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

    async fn send(&mut self, line: String) -> JfResult<()> {
        self.tx.send(line).await?;
        Ok(())
    }

    async fn send_with_guard(&mut self, log_level: LogLevel, msg: String) -> JfResult<()> {
        if self.log_level >= log_level {
            let line = log_generator::LogGenerator::new(log_level, msg).gen();
            self.send(line).await?;
        }
        Ok(())
    }

    pub async fn force<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send(line.as_ref().to_string()).await?;
        Ok(())
    }

    pub async fn error<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(LogLevel::Error, line.as_ref().to_string())
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn warn<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(LogLevel::Warn, line.as_ref().to_string())
            .await?;
        Ok(())
    }

    pub async fn info<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(LogLevel::Info, line.as_ref().to_string())
            .await?;
        Ok(())
    }

    pub async fn debug<S: AsRef<str>>(&mut self, line: S) -> JfResult<()> {
        self.send_with_guard(LogLevel::Debug, line.as_ref().to_string())
            .await?;
        Ok(())
    }
}
