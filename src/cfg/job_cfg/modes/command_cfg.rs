#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommandCfg {
    #[serde(flatten)]
    pub common: super::super::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::CommandParams,
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: CommandCfg = toml::from_str(
            r#"
private = true
description = "test-desc"
command = "test"
args = ["test1", "test2"]
            "#,
        )?;

        assert!(cfg.common.private());
        assert_eq!(cfg.common.description(), "test-desc");
        assert_eq!(cfg.params.command, "test");
        assert_eq!(cfg.params.args, vec!["test1", "test2"]);
        Ok(())
    }
}
