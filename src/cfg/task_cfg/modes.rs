use serde::Deserialize;

use super::common::CommonCfg;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::CommandParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParallelCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::ParallelParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SequentialCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::SequentialParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShellCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::ShellParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::WatchParams,
}
