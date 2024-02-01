#[cfg(test)]
mod mock_writer;
mod stdout;

#[cfg(test)]
pub use mock_writer::MockLogWriter;
pub use stdout::JfStdout;

use crate::util::error::JfResult;

#[async_trait::async_trait]
pub trait LogWriter: Send + Sync + Clone + 'static {
    async fn write(&mut self, str: &str) -> JfResult<()>;
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::async_test;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn init() {
        let _ = JfStdout::new();
        let _ = MockLogWriter::new();
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn write() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut w = MockLogWriter::new();
                w.write("test").await?;
                assert_eq!(w.lines(), vec!["test"]);
                w.write("test2").await?;
                assert_eq!(w.lines(), vec!["test", "test2"]);
                Ok(())
            },
        )
    }
}
