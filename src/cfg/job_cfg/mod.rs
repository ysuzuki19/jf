// SPDX-License-Identifier: MPL-2.0
mod common;
mod deserialize;
mod modes;
mod visibility;

pub use self::visibility::Visibility;
#[cfg(test)]
pub use common::CommonCfg;
#[cfg(test)]
pub use modes::MockCfg;
#[cfg(test)]
pub use modes::WatchCfg;

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

#[cfg(test)]
mod fixtures {
    use crate::util::{error::JfResult, testutil::TryFixture};

    use super::*;

    const CFG_CONTENT: &str = r#"
mode = "mock"
each_sleep_time = 100
sleep_count = 3
"#;

    impl TryFixture for JobCfg {
        #[coverage(off)]
        fn try_fixture() -> JfResult<Self> {
            Ok(toml::from_str(CFG_CONTENT)?)
        }
    }
}
