use crate::logging::Logger;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Ctx {
    logger: Logger,
}

impl Ctx {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub fn logger(&self) -> Logger {
        self.logger.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{logging::LoggingMock, util::testutil::*};

    use super::*;

    impl AsyncFixture for Ctx {
        #[cfg_attr(coverage, coverage(off))]
        async fn async_fixture() -> Self {
            let logging_mock = LoggingMock::new().await;
            Self {
                logger: logging_mock.logger,
            }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cover() {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let _ = Ctx::async_fixture().await;
            },
        );
    }
}
