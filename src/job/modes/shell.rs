use crate::{
    error::JfResult,
    job::{Job, Runner},
};

#[derive(Debug, Clone, serde::Deserialize)]
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

    async fn cancel(&self) -> JfResult<()> {
        self.command.cancel().await?;
        Ok(())
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
    use crate::testutil::Fixture;

    use super::*;

    impl Fixture for ShellParams {
        #[coverage(off)]
        fn gen() -> Self {
            ShellParams {
                script: "echo hello".to_string(),
                args: None,
            }
        }
    }

    impl Fixture for Shell {
        #[coverage(off)]
        fn gen() -> Self {
            Shell::new(Fixture::gen())
        }
    }

    #[tokio::test]
    #[coverage(off)]
    async fn run_without_blocking() -> JfResult<()> {
        let shell = Shell::gen();
        shell.start().await?;
        assert!(!shell.is_finished().await?);
        assert!(!shell.command.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    #[coverage(off)]
    async fn wait() -> JfResult<()> {
        let shell = Shell::gen().start().await?;
        shell.wait().await?;
        assert!(shell.is_finished().await?);
        assert!(shell.command.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    #[coverage(off)]
    async fn cancel() -> JfResult<()> {
        let shell = Shell::gen().start().await?;
        shell.cancel().await?;
        assert!(shell.is_finished().await?);
        assert!(shell.command.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    #[coverage(off)]
    async fn bunshin() -> JfResult<()> {
        let origin = Shell::gen().start().await?;
        origin.cancel().await?;
        assert!(origin.is_finished().await?);
        let bunshin = origin.bunshin();
        assert!(!bunshin.is_finished().await?);
        Ok(())
    }

    #[tokio::test]
    #[coverage(off)]
    async fn is_finished_not_yet_started() -> JfResult<()> {
        let shell = Shell::gen();
        assert!(!shell.is_finished().await?);
        Ok(())
    }
}
