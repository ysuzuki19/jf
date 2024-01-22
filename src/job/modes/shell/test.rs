use crate::testutil::*;

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
            let shell = Shell::fixture();
            shell.start().await?.wait().await?;
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
