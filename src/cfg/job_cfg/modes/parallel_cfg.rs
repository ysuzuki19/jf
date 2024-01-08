#[derive(serde::Deserialize)]
pub struct ParallelCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::ParallelParams,
}

#[cfg(test)]
pub mod fixtures {
    pub const SIMPLE: &str = r#"
description = "test-desc"
jobs = ["test-job1", "test-job2"]"#;
    pub const JOBS: &[&str] = &["test-job1", "test-job2"];
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn deserialize() -> JfResult<()> {
        let cfg: ParallelCfg = toml::from_str(fixtures::SIMPLE)?;

        assert_eq!(cfg.params.jobs, fixtures::JOBS);
        Ok(())
    }
}
