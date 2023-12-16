#[derive(Debug, Clone, serde::Deserialize)]
pub struct SequentialCfg {
    #[serde(flatten)]
    pub common: super::super::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::SequentialParams,
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: SequentialCfg = toml::from_str(
            r#"
private = false
description = "test-desc"
jobs = ["test"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test-desc");
        assert_eq!(cfg.params.jobs, vec!["test"]);
        Ok(())
    }
}