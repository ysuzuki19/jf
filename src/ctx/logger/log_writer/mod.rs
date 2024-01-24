#[cfg(test)]
mod mock_writer;
mod stdout;

#[cfg(test)]
pub use mock_writer::MockLogWriter;
pub use stdout::JfStdout;

use crate::error::JfResult;

#[async_trait::async_trait]
pub trait LogWriter: Send + Sync + Clone + 'static {
    fn initialize() -> Self;
    async fn write(&mut self, str: &str) -> JfResult<()>;
}

#[cfg(test)]
mod tests {
    use crate::testutil::async_test;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn initialize() {
        let _ = JfStdout::initialize();
        let _ = MockLogWriter::initialize();
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn write() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut w = MockLogWriter::initialize();
                w.write("test").await?;
                assert_eq!(w.lines.len(), 1);
                assert_eq!(w.lines[0], "test");
                w.write("test2").await?;
                assert_eq!(w.lines.len(), 2);
                assert_eq!(w.lines[0], "test");
                assert_eq!(w.lines[1], "test2");
                Ok(())
            },
        )
    }
}
