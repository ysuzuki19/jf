use serde::Deserialize;

use super::common::CommonConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandConfig {
    #[serde(flatten)]
    pub common: CommonConfig,
    #[serde(flatten)]
    pub params: crate::task::modes::CommandParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParallelConfig {
    #[serde(flatten)]
    pub common: CommonConfig,
    #[serde(flatten)]
    pub params: crate::task::modes::ParallelParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SequentialConfig {
    #[serde(flatten)]
    pub common: CommonConfig,
    #[serde(flatten)]
    pub params: crate::task::modes::SequentialParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShellConfig {
    #[serde(flatten)]
    pub common: CommonConfig,
    #[serde(flatten)]
    pub params: crate::task::modes::ShellParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchConfig {
    #[serde(flatten)]
    pub common: CommonConfig,
    #[serde(flatten)]
    pub params: crate::task::modes::WatchParams,
}
