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
    Command(Command),
    Shell(Shell),
    Parallel(Parallel),
    Sequential(Sequential),
    Watch(Watch),
}

impl TaskConfig {
    pub fn private(&self) -> bool {
        match self {
            TaskConfig::Command(c) => c.common.private,
            TaskConfig::Shell(s) => s.common.private,
            TaskConfig::Parallel(p) => p.common.private,
            TaskConfig::Sequential(s) => s.common.private,
            TaskConfig::Watch(w) => w.common.private,
        }
    }

    pub fn description(&self) -> String {
        match self {
            TaskConfig::Command(c) => c.common.description.clone(),
            TaskConfig::Shell(s) => s.common.description.clone(),
            TaskConfig::Parallel(p) => p.common.description.clone(),
            TaskConfig::Sequential(s) => s.common.description.clone(),
            TaskConfig::Watch(w) => w.common.description.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    #[serde(flatten)]
    pub params: crate::task::modes::CommandParams,
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
pub struct Watch {
    #[serde(flatten)]
    pub params: crate::task::modes::WatchParams,
    #[serde(flatten)]
    pub common: CommonConfig,
}

impl<'de> Deserialize<'de> for TaskConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // temp value to deserialize into TaskConfig by `mode`
        let value = serde_json::Value::deserialize(deserializer)?;

        // match `mode` value if it exists
        // otherwise default to `command`
        match value
            .get("mode")
            .and_then(|m| m.as_str())
            .unwrap_or("command")
        {
            "command" => Ok(TaskConfig::Command(
                Command::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "shell" => Ok(TaskConfig::Shell(
                Shell::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "parallel" => Ok(TaskConfig::Parallel(
                Parallel::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "sequential" => Ok(TaskConfig::Sequential(
                Sequential::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "watch" => Ok(TaskConfig::Watch(
                Watch::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            m => Err(serde::de::Error::custom(format!("Unknown mode: {m}"))),
        }
    }
}
