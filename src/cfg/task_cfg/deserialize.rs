use super::{
    modes::{CommandCfg, ParallelCfg, SequentialCfg, ShellCfg, WatchCfg},
    TaskCfg,
};

impl<'de> serde::Deserialize<'de> for TaskCfg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // temp value to deserialize into TaskCfg by `mode`
        let value = serde_json::Value::deserialize(deserializer)?;

        // match `mode` value if it exists
        // otherwise default to `command`
        match value
            .get("mode")
            .and_then(|m| m.as_str())
            .unwrap_or("command")
        {
            "command" => Ok(Self::Command(
                CommandCfg::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "parallel" => Ok(Self::Parallel(
                ParallelCfg::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "sequential" => Ok(Self::Sequential(
                SequentialCfg::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "shell" => Ok(Self::Shell(
                ShellCfg::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "watch" => Ok(Self::Watch(
                WatchCfg::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            m => Err(serde::de::Error::custom(format!("Unknown mode: {m}"))),
        }
    }
}
