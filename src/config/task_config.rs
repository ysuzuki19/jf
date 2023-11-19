mod common;
mod deserialize;
mod modes;

#[derive(Debug, Clone)]
pub enum TaskConfig {
    Command(modes::CommandConfig),
    Parallel(modes::ParallelConfig),
    Sequential(modes::SequentialConfig),
    Shell(modes::ShellConfig),
    Watch(modes::WatchConfig),
}

impl TaskConfig {
    pub fn private(&self) -> bool {
        match self {
            TaskConfig::Command(c) => c.common.private(),
            TaskConfig::Parallel(p) => p.common.private(),
            TaskConfig::Sequential(s) => s.common.private(),
            TaskConfig::Shell(s) => s.common.private(),
            TaskConfig::Watch(w) => w.common.private(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            TaskConfig::Command(c) => c.common.description(),
            TaskConfig::Parallel(p) => p.common.description(),
            TaskConfig::Sequential(s) => s.common.description(),
            TaskConfig::Shell(s) => s.common.description(),
            TaskConfig::Watch(w) => w.common.description(),
        }
    }
}
