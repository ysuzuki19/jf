use crate::util::testutil::*;

use super::*;

impl Fixture for Shell<MockLogWriter> {
    #[cfg_attr(coverage, coverage(off))]
    fn fixture() -> Self {
        let params = ShellParams {
            script: "echo hello".to_string(),
            args: None,
        };
        Shell::new(params)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_without_blocking() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let shell = Shell::fixture();
            shell.start(Fixture::fixture()).await?;
            assert!(!shell.is_finished().await?);
            assert!(!shell.command.is_finished().await?);
            Ok(())
        },
    )
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn join() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let shell = Shell::fixture();
            shell.start(Fixture::fixture()).await?.join().await?;
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
            shell.start(Fixture::fixture()).await?.cancel().await?;
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
            origin.start(Fixture::fixture()).await?.cancel().await?;
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
