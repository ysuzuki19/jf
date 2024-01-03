mod configured;
mod statics;

use crate::error::JfResult;

pub use self::configured::Configured;
pub use self::statics::Statics;

use super::{Ctx, Opts};

#[async_trait::async_trait]
pub trait CliAction {
    async fn run(self, ctx: Ctx, opts: Opts) -> JfResult<()>;
}

#[cfg_attr(test, derive(Debug, PartialEq))]
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
impl crate::testutil::Fixture for Action {
    fn fixture() -> Self {
        Action::Statics(Statics::fixture())
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::tuple_fixture;

    use super::*;

    #[tokio::test]
    async fn help() -> JfResult<()> {
        let s = Action::Statics(Statics::Help);
        let (ctx, opts) = tuple_fixture();
        s.run(ctx, opts).await?;
        Ok(())
    }

    #[tokio::test]
    async fn run() -> JfResult<()> {
        let c = Action::Configured(Configured::Run(String::from("test-fixture")));
        let (ctx, opts) = tuple_fixture();
        c.run(ctx, opts).await?;
        Ok(())
    }
}
