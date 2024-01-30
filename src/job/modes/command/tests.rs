use crate::testutil::*;

use super::*;

impl Fixture for Command<MockLogWriter> {
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
            command.start(Fixture::fixture()).await?;
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
            let command = Command::fixture();
            command.start(Fixture::fixture()).await?;
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
            let command = Command::fixture();
            command.start(Fixture::fixture()).await?.cancel().await?;
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
            let origin = Command::fixture();
            origin.start(Fixture::fixture()).await?;
            origin.join().await?;
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
            let command = Command::fixture();
            assert!(!command.is_finished().await?);
            Ok(())
        },
    )
}
