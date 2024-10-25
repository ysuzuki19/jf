// SPDX-License-Identifier: MPL-2.0
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
#[coverage(off)]
fn run_without_blocking() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn join() -> JfResult<()> {
    async_test(
        #[coverage(off)]
        async {
            let shell = Shell::async_fixture().await;
            shell.start().await?;
            assert!(shell.join().await?.is_succeed());
            assert!(shell.is_finished().await?);
            assert!(shell.command.is_finished().await?);
            Ok(())
        },
    )
}

#[test]
#[coverage(off)]
fn cancel() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn bunshin() -> JfResult<()> {
    async_test(
        #[coverage(off)]
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
#[coverage(off)]
fn is_finished_not_yet_started() -> JfResult<()> {
    async_test(
        #[coverage(off)]
        async {
            let shell = Shell::async_fixture().await;
            assert!(!shell.is_finished().await?);
            Ok(())
        },
    )
}
