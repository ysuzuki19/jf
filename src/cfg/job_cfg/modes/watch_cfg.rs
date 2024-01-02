#[derive(Debug, Clone, serde::Deserialize)]
pub struct WatchCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
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
job = "test-job"
watch_list = ["test1", "./src/**/*.rs"]
"#,
        )?;

        assert_eq!(cfg.params.job, "test-job");
        assert_eq!(cfg.params.watch_list, vec!["test1", "./src/**/*.rs"]);
        Ok(())
    }
}
