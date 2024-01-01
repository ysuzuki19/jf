mod common;
mod deserialize;
mod modes;
mod visibility;

pub use self::visibility::Visibility;

#[derive(Debug, Clone)]
pub enum JobCfg {
    Command(modes::CommandCfg),
    Parallel(modes::ParallelCfg),
    Sequential(modes::SequentialCfg),
    Shell(modes::ShellCfg),
    Watch(modes::WatchCfg),
    #[cfg(test)]
    Mock(modes::MockCfg),
}

impl JobCfg {
    pub fn visibility(&self) -> &Visibility {
        match self {
            JobCfg::Command(c) => c.common.visibility(),
            JobCfg::Parallel(p) => p.common.visibility(),
            JobCfg::Sequential(s) => s.common.visibility(),
            JobCfg::Shell(s) => s.common.visibility(),
            JobCfg::Watch(w) => w.common.visibility(),
            #[cfg(test)]
            JobCfg::Mock(m) => m.common.visibility(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            JobCfg::Command(c) => c.common.description(),
            JobCfg::Parallel(p) => p.common.description(),
            JobCfg::Sequential(s) => s.common.description(),
            JobCfg::Shell(s) => s.common.description(),
            JobCfg::Watch(w) => w.common.description(),
            #[cfg(test)]
            JobCfg::Mock(m) => m.common.description(),
        }
    }
}
