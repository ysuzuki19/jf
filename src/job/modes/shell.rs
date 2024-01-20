use crate::{
    error::JfResult,
    job::{Job, Runner},
};

#[derive(Clone, serde::Deserialize)]
pub struct ShellParams {
    pub script: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Shell {
    params: ShellParams,
    command: super::Command,
}

impl Shell {
    pub fn new(params: ShellParams) -> Self {
        let mut args = params.args.clone().unwrap_or_default();
        args.extend(vec!["-c".to_string(), params.script.clone()]);
        let command = super::Command::new(super::CommandParams {
            command: "sh".to_string(),
            args,
        });
        Self { params, command }
    }
}

#[async_trait::async_trait]
impl Runner for Shell {
    async fn start(&self) -> JfResult<Self> {
        self.command.start().await?;
        Ok(self.clone())
    }

    async fn is_finished(&self) -> JfResult<bool> {
        self.command.is_finished().await
    }

    async fn cancel(&self) -> JfResult<Self> {
        self.command.cancel().await?;
        Ok(self.clone())
    }

    fn bunshin(&self) -> Self {
        let command = self.command.bunshin();
        Self {
            params: self.params.clone(),
            command,
        }
    }
}

impl From<Shell> for Job {
    fn from(value: Shell) -> Self {
        Self::Shell(value)
    }
}

#[cfg(test)]
mod test {
    use crate::testutil::{async_test, Fixture};

    use super::*;

    impl Fixture for ShellParams {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            ShellParams {
                script: "echo hello".to_string(),
                args: None,
            }
        }
    }

    impl Fixture for Shell {
        #[cfg_attr(coverage, coverage(off))]
        fn fixture() -> Self {
            Shell::new(Fixture::fixture())
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn run_without_blocking() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let shell = Shell::fixture();
                shell.start().await?;
                assert!(!shell.is_finished().await?);
                assert!(!shell.command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn wait() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let shell = Shell::fixture().start().await?;
                shell.wait().await?;
                assert!(shell.is_finished().await?);
                assert!(shell.command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn cancel() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let shell = Shell::fixture();
                shell.start().await?.cancel().await?;
                assert!(shell.is_finished().await?);
                assert!(shell.command.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn bunshin() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let origin = Shell::fixture();
                origin.start().await?.cancel().await?;
                assert!(origin.is_finished().await?);
                let bunshin = origin.bunshin();
                assert!(!bunshin.is_finished().await?);
                Ok(())
            },
        )
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn is_finished_not_yet_started() -> JfResult<()> {
        async_test(
            #[cfg_attr(coverage, coverage(off))]
            async {
                let shell = Shell::fixture();
                assert!(!shell.is_finished().await?);
                Ok(())
            },
        )
    }
}
