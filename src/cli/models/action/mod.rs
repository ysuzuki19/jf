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
