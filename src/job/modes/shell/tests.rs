use crate::util::testutil::*;

use super::*;

impl AsyncFixture for Shell {
    async fn async_fixture() -> Self {
        let params = ShellParams {
            script: "echo hello".to_string(),
            args: None,
        };
        Shell::new(Ctx::async_fixture().await, params)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_without_blocking() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let shell = Shell::async_fixture().await;
            shell.start().await?;
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
            let shell = Shell::async_fixture().await;
            shell.start().await?.join().await?;
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
            let shell = Shell::async_fixture().await;
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
            let origin = Shell::async_fixture().await;
            origin.start().await?.cancel().await?;
            assert!(origin.is_finished().await?);
            let bunshin = origin.bunshin().await;
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
            let shell = Shell::async_fixture().await;
            assert!(!shell.is_finished().await?);
            Ok(())
        },
    )
}
