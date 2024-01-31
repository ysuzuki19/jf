use tokio::io::AsyncWriteExt;

use crate::util::error::JfResult;

use super::LogWriter;

pub struct JfStdout(tokio::io::Stdout);

impl Clone for JfStdout {
    fn clone(&self) -> Self {
        Self(tokio::io::stdout())
    }
}

#[async_trait::async_trait]
impl LogWriter for JfStdout {
    fn init() -> Self {
        Self(tokio::io::stdout())
    }

    #[cfg_attr(coverage, coverage(off))]
    async fn write(&mut self, s: &str) -> JfResult<()> {
        // let now = Local::now().format("%H:%M:%S.%3f");
        // let line = format!("[{}] {}", now, s);
        let line = s.to_string();
        self.0.write_all(line.as_bytes()).await?;
        #[cfg(not(test))]
        self.0.write_all(b"\n").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::async_test;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async move {
                let mut js = JfStdout::init();
                js.write("").await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn instance() {
        let js = JfStdout::init();
        let _ = js.clone();
    }
}
