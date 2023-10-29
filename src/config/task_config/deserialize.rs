use super::TaskConfig;

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
            "command" => Ok(TaskConfig::Command(
                super::modes::Command::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "parallel" => Ok(TaskConfig::Parallel(
                super::modes::Parallel::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "sequential" => Ok(TaskConfig::Sequential(
                super::modes::Sequential::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "shell" => Ok(TaskConfig::Shell(
                super::modes::Shell::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            "watch" => Ok(TaskConfig::Watch(
                super::modes::Watch::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            m => Err(serde::de::Error::custom(format!("Unknown mode: {m}"))),
        }
    }
}
