use crate::{
    cli::{
        completion_script,
        models::{Ctx, Opts},
        Args,
    },
    error::JfResult,
};

use super::{Action, CliAction};

// Action without job configuration
pub enum Statics {
    Completion(clap_complete::Shell),
    Help,
}

impl From<Statics> for Action {
    fn from(s: Statics) -> Self {
        Action::Statics(s)
    }
}

#[async_trait::async_trait]
impl CliAction for Statics {
    async fn run(self, ctx: Ctx, _: Opts) -> JfResult<()> {
        match self {
            Statics::Help => <Args as clap::CommandFactory>::command().print_help()?,
            Statics::Completion(shell) => ctx.logger.log(completion_script::generate(shell)),
        }
        Ok(())
    }
}