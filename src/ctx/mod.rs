// SPDX-License-Identifier: MPL-2.0
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
        async fn async_fixture() -> Self {
            let logging_mock = LoggingMock::new().await;
            Self {
                logger: logging_mock.logger,
            }
        }
    }

    #[test]
    #[coverage(off)]
    fn cover() {
        async_test(
            #[coverage(off)]
            async {
                let ctx = Ctx::async_fixture().await;
                println!("{:?}", ctx)
            },
        );
    }
}
