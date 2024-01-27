use crate::error::JfResult;

use super::LogWriter;

#[derive(Clone, PartialEq)]
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
