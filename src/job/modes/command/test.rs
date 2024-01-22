use crate::testutil::async_test;

use super::*;

#[cfg_attr(coverage, coverage(off))]
fn test_command_factory() -> Command {
    Command::new(CommandParams {
        command: String::from("sleep"),
        args: vec![String::from("1")],
    })
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_without_blocking() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let command = test_command_factory();
            command.start().await?;
            assert!(!command.is_finished().await?);
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
            let command = test_command_factory();
            command.start().await?;
            command.wait().await?;
            assert!(command.is_finished().await?);
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
            let command = test_command_factory();
            command.start().await?.cancel().await?;
            assert!(command.is_finished().await?);
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
            let command = test_command_factory().bunshin();
            command.start().await?.cancel().await?;
            assert!(command.is_finished().await?);
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
            let command = test_command_factory();
            assert!(!command.is_finished().await?);
            Ok(())
        },
    )
}
