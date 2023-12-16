mod common;
mod deserialize;
mod modes;

#[derive(Debug, Clone)]
pub enum JobCfg {
    Command(modes::CommandCfg),
    Parallel(modes::ParallelCfg),
    Sequential(modes::SequentialCfg),
    Shell(modes::ShellCfg),
    Watch(modes::WatchCfg),
}

impl JobCfg {
    pub fn private(&self) -> bool {
        match self {
            JobCfg::Command(c) => c.common.private(),
            JobCfg::Parallel(p) => p.common.private(),
            JobCfg::Sequential(s) => s.common.private(),
            JobCfg::Shell(s) => s.common.private(),
            JobCfg::Watch(w) => w.common.private(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            JobCfg::Command(c) => c.common.description(),
            JobCfg::Parallel(p) => p.common.description(),
            JobCfg::Sequential(s) => s.common.description(),
            JobCfg::Shell(s) => s.common.description(),
            JobCfg::Watch(w) => w.common.description(),
        }
    }
}
