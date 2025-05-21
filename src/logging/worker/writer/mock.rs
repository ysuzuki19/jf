// SPDX-License-Identifier: MPL-2.0
use std::sync::{Arc, Mutex};

use crate::util::error::JfResult;

use super::Writer;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Mock {
    lines: Arc<Mutex<Vec<String>>>,
}

impl Mock {
    pub fn new() -> Self {
        Self {
            lines: Arc::new(Mutex::new(vec![])),
        }
    }
}

#[async_trait::async_trait]
impl Writer for Mock {
    async fn write(&mut self, str: &str) -> JfResult<()> {
        self.lines.lock().as_mut().unwrap().push(str.to_string());
        Ok(())
    }
}

impl Mock {
    pub fn lines(&self) -> Vec<String> {
        self.lines.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::async_test;

    use super::*;

    #[test]
    #[coverage(off)]
    fn init() {
        async_test(
            #[coverage(off)]
            async move {
                let writer = Mock::new();
                assert_eq!(writer.lines(), Vec::<String>::new());
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn write() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async move {
                let mut writer = Mock::new();
                writer.write("test").await?;
                assert_eq!(writer.lines(), vec!["test".to_string()]);
                assert_eq!(writer.clone().lines(), vec!["test".to_string()]);
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn cover() {
        let writer = Mock::new();
        println!("{writer:?}");
    }
}
