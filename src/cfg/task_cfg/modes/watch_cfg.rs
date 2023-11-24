#[derive(Debug, Clone, serde::Deserialize)]
pub struct WatchCfg {
    #[serde(flatten)]
    pub common: super::super::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::WatchParams,
}

#[cfg(test)]
mod tests {
    use crate::error::CmdResult;

    use super::*;

    #[test]
    fn deserialize() -> CmdResult<()> {
        let cfg: WatchCfg = toml::from_str(
            r#"
private = false
description = "test2"
task = "test-task"
watch_list = ["test1", "./src/**/*.rs"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test2");
        assert_eq!(cfg.params.task, "test-task");
        assert_eq!(cfg.params.watch_list, vec!["test1", "./src/**/*.rs"]);
        Ok(())
    }
}
