use super::{
    modes::{CommandConfig, ParallelConfig, SequentialConfig, ShellConfig, WatchConfig},
    TaskConfig,
};

impl<'de> serde::Deserialize<'de> for TaskConfig {
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
            "command" => Ok(Self::Command(
                CommandConfig::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "parallel" => Ok(Self::Parallel(
                ParallelConfig::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "sequential" => Ok(Self::Sequential(
                SequentialConfig::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "shell" => Ok(Self::Shell(
                ShellConfig::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "watch" => Ok(Self::Watch(
                WatchConfig::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            m => Err(serde::de::Error::custom(format!("Unknown mode: {m}"))),
        }
    }
}
