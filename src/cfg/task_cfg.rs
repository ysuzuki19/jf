mod common;
mod deserialize;
mod modes;

#[derive(Debug, Clone)]
pub enum TaskCfg {
    Command(modes::CommandCfg),
    Parallel(modes::ParallelCfg),
    Sequential(modes::SequentialCfg),
    Shell(modes::ShellCfg),
    Watch(modes::WatchCfg),
}

impl TaskCfg {
    pub fn private(&self) -> bool {
        match self {
            TaskCfg::Command(c) => c.common.private(),
            TaskCfg::Parallel(p) => p.common.private(),
            TaskCfg::Sequential(s) => s.common.private(),
            TaskCfg::Shell(s) => s.common.private(),
            TaskCfg::Watch(w) => w.common.private(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            TaskCfg::Command(c) => c.common.description(),
            TaskCfg::Parallel(p) => p.common.description(),
            TaskCfg::Sequential(s) => s.common.description(),
            TaskCfg::Shell(s) => s.common.description(),
            TaskCfg::Watch(w) => w.common.description(),
        }
    }
}
