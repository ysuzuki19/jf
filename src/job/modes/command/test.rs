use crate::testutil::*;

use super::*;

impl Fixture for Command {
    #[cfg_attr(coverage, coverage(off))]
    fn fixture() -> Self {
        let params = CommandParams {
            command: String::from("sleep"),
            args: vec![String::from("1")],
        };
        Command::new(params)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_without_blocking() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let command = Command::fixture();
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
            let command = Command::fixture();
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
            let command = Command::fixture();
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
            let command = Command::fixture().bunshin();
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
            let command = Command::fixture();
            assert!(!command.is_finished().await?);
            Ok(())
        },
    )
}
