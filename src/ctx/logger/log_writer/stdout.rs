use tokio::io::AsyncWriteExt;

use crate::error::JfResult;

use super::LogWriter;

pub struct JfStdout(tokio::io::Stdout);

impl Clone for JfStdout {
    fn clone(&self) -> Self {
        Self(tokio::io::stdout())
    }
}

#[async_trait::async_trait]
impl LogWriter for JfStdout {
    fn initialize() -> Self {
        Self(tokio::io::stdout())
    }

    #[cfg_attr(coverage, coverage(off))]
    async fn write(&mut self, str: &str) -> JfResult<()> {
        self.0.write_all(str.as_bytes()).await?;
        self.0.write_all(b"\n").await?;
        #[cfg(test)]
        unreachable!("JfStdout::write should not be called in tests");
        #[cfg(not(test))]
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn instance() {
        let js = JfStdout::initialize();
        let _ = js.clone();
    }
}
