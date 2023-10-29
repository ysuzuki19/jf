use serde::Deserialize;

use super::CommonConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    #[serde(flatten)]
    pub params: crate::task::modes::CommandParams,
    #[serde(flatten)]
    pub common: CommonConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Parallel {
    #[serde(flatten)]
    pub params: crate::task::modes::ParallelParams,
    #[serde(flatten)]
    pub common: CommonConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sequential {
    #[serde(flatten)]
    pub params: crate::task::modes::SequentialParams,
    #[serde(flatten)]
    pub common: CommonConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Shell {
    #[serde(flatten)]
    pub params: crate::task::modes::ShellParams,
    #[serde(flatten)]
    pub common: CommonConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Watch {
    #[serde(flatten)]
    pub params: crate::task::modes::WatchParams,
    #[serde(flatten)]
    pub common: CommonConfig,
}
