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
mod tests {
    use crate::testutil::{async_test, Fixture};

    use super::*;

    impl Fixture for Statics {
        #[cfg_attr(coverage, coverage(off))]
        fn gen() -> Self {
            Statics::Help
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn fixture() {
        let s = Statics::gen();
        assert_eq!(s, Statics::Help);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn completion() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let s = Statics::Completion(clap_complete::Shell::Bash);
                s.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn help() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let s = Statics::Help;
                s.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn version() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let s = Statics::Version;
                s.run(Fixture::gen(), Fixture::gen()).await?;
                Ok(())
            },
        )
    }
}
