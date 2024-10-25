// SPDX-License-Identifier: MPL-2.0
use tokio::io::AsyncWriteExt;

use crate::util::error::JfResult;

use super::Writer;

pub struct Stdout(tokio::io::Stdout);

impl Clone for Stdout {
    fn clone(&self) -> Self {
        Self(tokio::io::stdout())
    }
}

impl Stdout {
    pub fn new() -> Self {
        Self(tokio::io::stdout())
    }
}

#[async_trait::async_trait]
impl Writer for Stdout {
    #[coverage(off)]
    async fn write(&mut self, s: &str) -> JfResult<()> {
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
    #[coverage(off)]
    fn cover() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async move {
                let mut js = Stdout::new();
                js.write("").await?;
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn instance() {
        let js = Stdout::new();
        let _ = js.clone();
    }
}
