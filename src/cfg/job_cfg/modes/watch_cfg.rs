#[derive(serde::Deserialize)]
pub struct WatchCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::WatchParams,
}

#[cfg(test)]
pub mod fixtures {
    pub const SIMPLE: &str = r#"
job = "test-job"
watch_list = ["test1", "./src/**/*.rs"]"#;
    pub const JOB: &str = "test-job";
    pub const WATCH_LIST: &[&str] = &["test1", "./src/**/*.rs"];
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn deserialize() -> JfResult<()> {
        let cfg: WatchCfg = toml::from_str(fixtures::SIMPLE)?;

        assert_eq!(cfg.params.job, fixtures::JOB);
        assert_eq!(cfg.params.watch_list, fixtures::WATCH_LIST);
        Ok(())
    }
}
