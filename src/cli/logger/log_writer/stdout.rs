use tokio::io::{AsyncWriteExt, Stdout};

use crate::error::JfResult;

use super::LogWriter;

#[async_trait::async_trait]
impl LogWriter for Stdout {
    fn initialize() -> Stdout {
        tokio::io::stdout()
    }

    async fn write(&mut self, str: &str) -> JfResult<()> {
        self.write_all(str.as_bytes()).await?;
        self.write_all(b"\n").await?;
        Ok(())
    }
}
