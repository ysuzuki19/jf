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

        if let JobCfg::Command(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Command");
        }
    }

    #[test]
    fn command() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "command"
command = "echo"
"#,
        )?;

        if let JobCfg::Command(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Command");
        }
    }

    #[test]
    fn parallel() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "parallel"
jobs = ["test", "test2"]
"#,
        )?;

        if let JobCfg::Parallel(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Parallel");
        }
    }

    #[test]
    fn sequential() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "sequential"
jobs = ["test", "test2"]
"#,
        )?;

        if let JobCfg::Sequential(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Sequential");
        }
    }

    #[test]
    fn shell() -> JfResult<()> {
        let cfg: JobCfg = toml::from_str(
            r#"
mode = "shell"
script = "echo hello"
"#,
        )?;

        if let JobCfg::Shell(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Shell");
        }
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

        if let JobCfg::Watch(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Watch");
        }
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

        if let JobCfg::Mock(_) = cfg {
            Ok(())
        } else {
            unreachable!("expected JobCfg::Mock");
        }
    }

    #[test]
    fn unknown() -> JfResult<()> {
        match toml::from_str::<JobCfg>(r#"mode = "unknown""#) {
            Err(_) => Ok(()),
            _ => unreachable!("expected error"),
        }
    }
}
