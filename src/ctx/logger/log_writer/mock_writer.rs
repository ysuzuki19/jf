use std::sync::{Arc, Mutex};

use crate::util::error::JfResult;

use super::LogWriter;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct MockLogWriter {
    lines: Arc<Mutex<Vec<String>>>,
}

impl PartialEq for MockLogWriter {
    fn eq(&self, other: &Self) -> bool {
        self.lines.lock().unwrap().clone() == other.lines.lock().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl LogWriter for MockLogWriter {
    fn init() -> Self {
        Self {
            lines: Arc::new(Mutex::new(vec![])),
        }
    }

    async fn write(&mut self, str: &str) -> JfResult<()> {
        self.lines.lock().as_mut().unwrap().push(str.to_string());
        Ok(())
    }
}

impl MockLogWriter {
    pub fn lines(&self) -> Vec<String> {
        self.lines.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::async_test;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn init() {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async move {
                let writer = MockLogWriter::init();
                assert_eq!(writer.lines(), Vec::<String>::new());
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn write() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async move {
                let mut writer = MockLogWriter::init();
                writer.write("test").await?;
                assert_eq!(writer.lines(), vec!["test".to_string()]);
                assert_eq!(writer.clone().lines(), vec!["test".to_string()]);
                Ok(())
            },
        )
    }
}
