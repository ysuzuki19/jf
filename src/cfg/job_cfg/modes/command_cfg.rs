#[derive(Clone, serde::Deserialize)]
pub struct CommandCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::CommandParams,
}

#[cfg(test)]
pub mod fixtures {
    pub const COMMAND: &str = "test";

    pub const SIMPLE: &str = r#"command = "test""#;

    pub const ARGS: &[&str] = &["test1", "test2"];

    pub const COMMAND_WITH_ARGS: &str = r#"
command = "test"
args = ["test1", "test2"]"#;
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: CommandCfg = toml::from_str(fixtures::SIMPLE)?;
        assert_eq!(cfg.params.command, fixtures::COMMAND);
        assert_eq!(cfg.params.args, Vec::<String>::new());

        let cfg: CommandCfg = toml::from_str(fixtures::COMMAND_WITH_ARGS)?;
        assert_eq!(cfg.params.command, fixtures::COMMAND);
        assert_eq!(cfg.params.args, fixtures::ARGS);

        Ok(())
    }
}
