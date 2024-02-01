use crate::util::testutil::*;

use super::*;

impl AsyncFixture for Command {
    #[cfg_attr(coverage, coverage(off))]
    async fn async_fixture() -> Self {
        let params = CommandParams {
            command: String::from("sleep"),
            args: vec![String::from("1")],
        };
        Command::new(Ctx::async_fixture().await, params)
    }
}

#[test]
#[cfg_attr(coverage, coverage(off))]
fn run_without_blocking() -> JfResult<()> {
    async_test(
        #[cfg_attr(coverage, coverage(off))]
        async {
            let command = Command::async_fixture().await;
            command.start().await?;
            assert!(!command.is_finished().await?);
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
            let command = Command::async_fixture().await;
            command.start().await?;
            command.join().await?;
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
            let command = Command::async_fixture().await;
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
            let origin = Command::async_fixture().await;
            origin.start().await?;
            origin.join().await?;
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
            let command = Command::async_fixture().await;
            assert!(!command.is_finished().await?);
            Ok(())
        },
    )
}
