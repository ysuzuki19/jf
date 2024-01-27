use crate::error::JfResult;

use super::LogWriter;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct MockLogWriter {
    pub lines: Vec<String>,
}

#[async_trait::async_trait]
impl LogWriter for MockLogWriter {
    fn init() -> Self {
        Self { lines: vec![] }
    }

    async fn write(&mut self, str: &str) -> JfResult<()> {
        self.lines.push(str.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::async_test;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn init() {
        let writer = MockLogWriter::init();
        assert_eq!(writer, LogWriter::init());
        assert_eq!(writer.lines, Vec::<String>::new());
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn write() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async move {
                let mut writer = MockLogWriter::init();
                writer.write("test").await?;
                assert_eq!(writer.lines, vec!["test".to_string()]);
                Ok(())
            },
        )
    }
}
