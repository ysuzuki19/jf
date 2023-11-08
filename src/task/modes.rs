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

pub use command::Params as CommandParams;
pub use parallel::Params as ParallelParams;
pub use sequential::Params as SequentialParams;
pub use shell::Params as ShellParams;
pub use watch::Params as WatchParams;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub(super) use mock::Mock;
