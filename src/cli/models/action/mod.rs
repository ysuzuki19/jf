mod configured;
mod statics;

use crate::ctx::Ctx;
use crate::util::error::JfResult;

pub use self::configured::Configured;
pub use self::statics::Statics;

use super::Opts;

#[async_trait::async_trait]
pub trait CliAction {
    async fn run(self, ctx: Ctx, opts: Opts) -> JfResult<()>;
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Action {
    Statics(Statics),
    Configured(Configured),
}

#[async_trait::async_trait]
impl CliAction for Action {
    async fn run(self, ctx: Ctx, opts: Opts) -> JfResult<()> {
        match self {
            Action::Statics(s) => s.run(ctx, opts).await,
            Action::Configured(c) => c.run(ctx, opts).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::testutil::*;

    use super::*;

    impl Fixture for Action {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Action::Statics(Fixture::fixture())
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn help() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let s = Action::Statics(Statics::Help);
                s.run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn run() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let c = Action::Configured(Configured::Run(String::from("test-fixture")));
                c.run(Ctx::async_fixture().await, Fixture::fixture())
                    .await?;
                Ok(())
            },
        )
    }
}
