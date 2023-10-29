use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CmdConfig {
    #[serde(rename = "task")]
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub mode: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub script: Option<String>,
    pub tasks: Option<Vec<String>>,
    pub task: Option<String>,
    pub watch_list: Option<Vec<String>>,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub description: String,
}

impl TaskConfig {
    pub fn into_pruned(self) -> (CommonConfig, RunnerConfig) {
        (
            CommonConfig {
                private: self.private,
                description: self.description,
            },
            RunnerConfig {
                mode: self.mode,
                command: self.command,
                args: self.args,
                tasks: self.tasks,
                task: self.task,
                watch_list: self.watch_list,
                script: self.script,
            },
        )
    }
}

pub struct CommonConfig {
    pub private: bool,
    pub description: String,
}

// TaskConfig without common fields
#[derive(Clone)]
pub struct RunnerConfig {
    pub mode: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub script: Option<String>,
    pub tasks: Option<Vec<String>>,
    pub task: Option<String>,
    pub watch_list: Option<Vec<String>>,
}
