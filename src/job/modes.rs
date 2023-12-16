mod command;
mod parallel;
mod sequential;
mod shell;
mod watch;

pub(super) use command::Command;
pub(super) use parallel::Parallel;
pub(super) use sequential::Sequential;
pub(super) use shell::Shell;
pub(super) use watch::Watch;

pub use command::CommandParams;
pub use parallel::ParallelParams;
pub use sequential::SequentialParams;
pub use shell::ShellParams;
pub use watch::WatchParams;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub(super) use mock::Mock;
