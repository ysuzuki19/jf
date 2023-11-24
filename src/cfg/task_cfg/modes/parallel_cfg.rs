#[derive(Debug, Clone, serde::Deserialize)]
pub struct ParallelCfg {
    #[serde(flatten)]
    pub common: super::super::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::ParallelParams,
}

#[cfg(test)]
mod tests {
    use crate::error::CmdResult;

    use super::*;

    #[test]
    fn deserialize() -> CmdResult<()> {
        let cfg: ParallelCfg = toml::from_str(
            r#"
private = false
description = "test"
tasks = ["test-task1", "test-task2"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test");
        assert_eq!(cfg.params.tasks, vec!["test-task1", "test-task2"]);
        Ok(())
    }
}
