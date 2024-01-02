pub mod command_cfg;
pub mod parallel_cfg;
pub mod sequential_cfg;
pub mod shell_cfg;
pub mod watch_cfg;

pub use command_cfg::CommandCfg;
pub use parallel_cfg::ParallelCfg;
pub use sequential_cfg::SequentialCfg;
pub use shell_cfg::ShellCfg;
pub use watch_cfg::WatchCfg;

#[cfg(test)]
mod mock_cfg;
#[cfg(test)]
pub use mock_cfg::MockCfg;

#[cfg(test)]
pub mod fixtures {
    pub use super::command_cfg::fixtures as command;
    pub use super::mock_cfg::fixtures as mock;
    pub use super::parallel_cfg::fixtures as parallel;
    pub use super::sequential_cfg::fixtures as sequential;
    pub use super::shell_cfg::fixtures as shell;
    pub use super::watch_cfg::fixtures as watch;
}
