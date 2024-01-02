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
    use crate::{cfg::job_cfg::modes, error::JfResult};

    use super::*;

    fn generate_modable_cfg(mode: &str, content: &str) -> String {
        format!(
            r#"mode = "{mode}"
{content}"#,
        )
    }

    #[test]
    fn default() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(modes::fixtures::command::SIMPLE)?;

        matches!(cfg, JobCfg::Command(_));
        Ok(())
    }

    #[test]
    fn command() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            generate_modable_cfg("command", modes::fixtures::command::COMMAND_WITH_ARGS).as_str(),
        )?;

        matches!(cfg, JobCfg::Command(_));
        Ok(())
    }

    #[test]
    fn parallel() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            generate_modable_cfg("parallel", modes::fixtures::parallel::SIMPLE).as_str(),
        )?;

        matches!(cfg, JobCfg::Parallel(_));
        Ok(())
    }

    #[test]
    fn sequential() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            generate_modable_cfg("sequential", modes::fixtures::sequential::SIMPLE).as_str(),
        )?;

        matches!(cfg, JobCfg::Sequential(_));
        Ok(())
    }

    #[test]
    fn shell() -> JfResult<()> {
        let cfg: JobCfg =
            toml::from_str(generate_modable_cfg("shell", modes::fixtures::shell::SIMPLE).as_str())?;

        matches!(cfg, JobCfg::Shell(_));
        Ok(())
    }

    #[test]
    fn watch() -> JfResult<()> {
        let cfg: JobCfg =
            toml::from_str(generate_modable_cfg("watch", modes::fixtures::watch::SIMPLE).as_str())?;

        matches!(cfg, JobCfg::Watch(_));
        Ok(())
    }

    #[test]
    fn mock() -> JfResult<()> {
        let cfg: JobCfg =
            toml::from_str(generate_modable_cfg("mock", modes::fixtures::mock::SIMPLE).as_str())?;

        matches!(cfg, JobCfg::Mock(_));
        Ok(())
    }

    #[test]
    fn unknown() {
        assert!(toml::from_str::<JobCfg>(generate_modable_cfg("unknown", "").as_str()).is_err());
    }
}
