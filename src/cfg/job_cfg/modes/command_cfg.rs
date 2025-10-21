// SPDX-License-Identifier: MPL-2.0
#[derive(serde::Deserialize)]
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

    pub const COMMAND_WITH_ENV: &str = r#"
command = "test"
args = []

[env]
MY_VAR = "my_value"
ANOTHER_VAR = "another_value"
"#;
}

#[cfg(test)]
mod tests {
    use crate::util::error::JfResult;

    use super::*;

    #[test]
    #[coverage(off)]
    fn deserialize() -> JfResult<()> {
        let cfg: CommandCfg = toml::from_str(fixtures::SIMPLE)?;
        assert_eq!(cfg.params.command, fixtures::COMMAND);
        assert_eq!(cfg.params.args, Vec::<String>::new());

        let cfg: CommandCfg = toml::from_str(fixtures::COMMAND_WITH_ARGS)?;
        assert_eq!(cfg.params.command, fixtures::COMMAND);
        assert_eq!(cfg.params.args, fixtures::ARGS);

        let cfg: CommandCfg = toml::from_str(fixtures::COMMAND_WITH_ENV)?;
        assert_eq!(cfg.params.command, fixtures::COMMAND);
        assert_eq!(cfg.params.env.get("MY_VAR"), Some(&"my_value".to_string()));
        assert_eq!(
            cfg.params.env.get("ANOTHER_VAR"),
            Some(&"another_value".to_string())
        );

        Ok(())
    }
}
