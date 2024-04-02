// SPDX-License-Identifier: MPL-2.0
#[cfg(test)]
mod mock;
mod stdout;

#[cfg(test)]
pub use mock::Mock;
pub use stdout::Stdout;

use crate::util::error::JfResult;

#[async_trait::async_trait]
pub trait Writer: Send + Sync + Clone + 'static {
    async fn write(&mut self, str: &str) -> JfResult<()>;
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::async_test;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn init() {
        let _ = Stdout::new();
        let _ = Mock::new();
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn write() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let mut w = Mock::new();
                w.write("test").await?;
                assert_eq!(w.lines(), vec!["test"]);
                w.write("test2").await?;
                assert_eq!(w.lines(), vec!["test", "test2"]);
                Ok(())
            },
        )
    }
}
