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
#[cfg_attr(test, derive(Debug, PartialEq))]
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
        let mut cmd = <Args as clap::CommandFactory>::command();
        let s = match self {
            Statics::Help => cmd.render_help().to_string(),
            Statics::Completion(shell) => completion_script::generate(shell),
        };
        ctx.logger.log(s);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures() -> (Ctx, Opts) {
        (Ctx::fixture(), Opts::fixture())
    }

    #[tokio::test]
    async fn help() -> JfResult<()> {
        let s = Statics::Help;
        let (ctx, opts) = fixtures();
        s.run(ctx, opts).await?;
        Ok(())
    }

    #[tokio::test]
    async fn completion() -> JfResult<()> {
        let s = Statics::Completion(clap_complete::Shell::Bash);
        let (ctx, opts) = fixtures();
        s.run(ctx, opts).await?;
        Ok(())
    }
}
