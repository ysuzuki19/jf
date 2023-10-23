mod command;
mod parallel;
mod sequential;
mod shell;
mod watch;

pub use command::Command;
pub use parallel::Parallel;
pub use sequential::Sequential;
pub use shell::Shell;
pub use watch::Watch;

use crate::error::CmdResult;

use super::runner::Context;

#[async_trait::async_trait]
pub trait Run {
    async fn run(&mut self, ctx: Context) -> CmdResult<()>;
}
