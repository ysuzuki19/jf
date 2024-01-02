use super::{
    modes::{CommandCfg, ParallelCfg, SequentialCfg, ShellCfg, WatchCfg},
    JobCfg,
};

impl<'de> serde::Deserialize<'de> for JobCfg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // temp value to deserialize into JobCfg by `mode`
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
            #[cfg(test)]
            "mock" => Ok(Self::Mock(
                super::modes::MockCfg::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            m => Err(serde::de::Error::custom(format!("Unknown mode: {m}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn default() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
command = "echo"
"#,
        )?;

        matches!(cfg, JobCfg::Command(_));
        Ok(())
    }

    #[test]
    fn command() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "command"
command = "echo"
"#,
        )?;

        matches!(cfg, JobCfg::Command(_));
        Ok(())
    }

    #[test]
    fn parallel() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "parallel"
jobs = ["test", "test2"]
"#,
        )?;

        matches!(cfg, JobCfg::Parallel(_));
        Ok(())
    }

    #[test]
    fn sequential() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "sequential"
jobs = ["test", "test2"]
"#,
        )?;

        matches!(cfg, JobCfg::Sequential(_));
        Ok(())
    }

    #[test]
    fn shell() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "shell"
script = "echo hello"
"#,
        )?;

        matches!(cfg, JobCfg::Shell(_));
        Ok(())
    }

    #[test]
    fn watch() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "watch"
job = "test"
watch_list = ["test", "test2"]
"#,
        )?;

        matches!(cfg, JobCfg::Watch(_));
        Ok(())
    }

    #[test]
    fn mock() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "mock"
each_sleep_time = 1
sleep_count = 3
"#,
        )?;

        matches!(cfg, JobCfg::Mock(_));
        Ok(())
    }

    #[test]
    fn unknown() {
        assert!(toml::from_str::<JobCfg>(r#"mode = "unknown""#).is_err());
    }
}
