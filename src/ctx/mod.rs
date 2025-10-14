// SPDX-License-Identifier: MPL-2.0
mod app_stack;
mod scopeout_log;

use crate::{
    ctx::{app_stack::AppStack, scopeout_log::ScopeoutLog},
    logging::{log_generator, LogLevel, Logger},
};

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Ctx {
    logger: Logger,
    app_stack: AppStack,
    verbose: bool,
}

// pub struct CtxSpanGuard<'a> {
//     ctx: &'a mut Ctx,
// }

// impl<'a> CtxSpanGuard<'a> {
//     pub fn new(ctx: &'a mut Ctx) -> Self {
//         Self { ctx }
//     }
// }

// impl Drop for CtxSpanGuard<'_> {
//     fn drop(&mut self) {
//         self.ctx.app_stack.pop();
//     }
// }

impl Ctx {
    pub fn new(logger: Logger, name: &str, verbose: bool) -> Self {
        Self {
            logger,
            app_stack: AppStack::new(name),
            verbose,
        }
    }

    pub fn logger(&self) -> Logger {
        self.logger.clone()
    }

    // pub fn with_span(&mut self, span: &str) -> CtxSpanGuard<'_> {
    //     self.app_stack.push(span);
    //     CtxSpanGuard::new(self)
    // }

    pub fn new_span(&self, span: &str) -> Self {
        let mut cloned = self.clone();
        cloned.app_stack.push(span);
        cloned
    }

    fn engine_log_message<S: AsRef<str>>(&self, msg: S) -> String {
        format!("[{}] {}", self.app_stack.stacked(), msg.as_ref())
    }

    pub fn engine_log<S: AsRef<str>>(&self, msg: S) {
        if self.verbose {
            let line =
                log_generator::LogGenerator::new(LogLevel::Debug, self.engine_log_message(msg))
                    .gen();
            println!("{line}");
        }
    }

    pub fn scopeout_engine_log<S: AsRef<str>>(&self, msg: S) -> ScopeoutLog {
        let msg = if self.verbose {
            Some(self.engine_log_message(msg.as_ref()))
        } else {
            None
        };
        ScopeoutLog::new(msg)
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
                app_stack: AppStack::new("test"),
                verbose: false,
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
                println!("{ctx:?}")
            },
        );
    }
}
