mod deserialize;
mod modes;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CmdConfig {
    #[serde(rename = "task")]
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommonConfig {
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum TaskConfig {
    Command(modes::Command),
    Parallel(modes::Parallel),
    Sequential(modes::Sequential),
    Shell(modes::Shell),
    Watch(modes::Watch),
}

impl TaskConfig {
    pub fn private(&self) -> bool {
        match self {
            TaskConfig::Command(c) => c.common.private,
            TaskConfig::Parallel(p) => p.common.private,
            TaskConfig::Sequential(s) => s.common.private,
            TaskConfig::Shell(s) => s.common.private,
            TaskConfig::Watch(w) => w.common.private,
        }
    }

    pub fn description(&self) -> String {
        match self {
            TaskConfig::Command(c) => c.common.description.clone(),
            TaskConfig::Parallel(p) => p.common.description.clone(),
            TaskConfig::Sequential(s) => s.common.description.clone(),
            TaskConfig::Shell(s) => s.common.description.clone(),
            TaskConfig::Watch(w) => w.common.description.clone(),
        }
    }
}
