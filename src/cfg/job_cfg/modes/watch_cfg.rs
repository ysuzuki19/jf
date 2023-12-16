#[derive(Debug, Clone, serde::Deserialize)]
pub struct WatchCfg {
    #[serde(flatten)]
    pub common: super::super::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::WatchParams,
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: WatchCfg = toml::from_str(
            r#"
private = false
description = "test2"
job = "test-job"
watch_list = ["test1", "./src/**/*.rs"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test2");
        assert_eq!(cfg.params.job, "test-job");
        assert_eq!(cfg.params.watch_list, vec!["test1", "./src/**/*.rs"]);
        Ok(())
    }
}
