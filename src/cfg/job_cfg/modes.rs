mod command_cfg;
mod parallel_cfg;
mod sequential_cfg;
mod shell_cfg;
mod watch_cfg;

pub use command_cfg::CommandCfg;
pub use parallel_cfg::ParallelCfg;
pub use sequential_cfg::SequentialCfg;
pub use shell_cfg::ShellCfg;
pub use watch_cfg::WatchCfg;

#[cfg(test)]
mod mock_cfg;
#[cfg(test)]
pub use mock_cfg::MockCfg;
