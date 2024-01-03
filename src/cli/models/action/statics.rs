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
    Version,
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
            Statics::Completion(shell) => completion_script::generate(shell),
            Statics::Help => cmd.render_help().to_string(),
            Statics::Version => cmd.render_version().to_string(),
        };
        ctx.logger.log(s);
        Ok(())
    }
}

#[cfg(test)]
impl crate::testutil::Fixture for Statics {
    fn fixture() -> Self {
        Statics::Help
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::{tuple_fixture, Fixture};

    use super::*;

    #[test]
    fn fixture() {
        let s = Statics::fixture();
        assert_eq!(s, Statics::Help);
    }

    #[tokio::test]
    async fn completion() -> JfResult<()> {
        let s = Statics::Completion(clap_complete::Shell::Bash);
        let (ctx, opts) = tuple_fixture();
        s.run(ctx, opts).await?;
        Ok(())
    }

    #[tokio::test]
    async fn help() -> JfResult<()> {
        let s = Statics::Help;
        let (ctx, opts) = tuple_fixture();
        s.run(ctx, opts).await?;
        Ok(())
    }

    #[tokio::test]
    async fn version() -> JfResult<()> {
        let s = Statics::Version;
        let (ctx, opts) = tuple_fixture();
        s.run(ctx, opts).await?;
        Ok(())
    }
}
