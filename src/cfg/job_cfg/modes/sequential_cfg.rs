// SPDX-License-Identifier: MPL-2.0
#[derive(serde::Deserialize)]
pub struct SequentialCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::SequentialParams,
}

#[cfg(test)]
pub mod fixtures {
    pub const SIMPLE: &str = r#"jobs = ["test-job1", "test-job2"]"#;
    pub const JOBS: &[&str] = &["test-job1", "test-job2"];
}

#[cfg(test)]
mod tests {
    use crate::util::error::JfResult;

    use super::*;

    #[test]
    #[coverage(off)]
    fn deserialize() -> JfResult<()> {
        let cfg: SequentialCfg = toml::from_str(fixtures::SIMPLE)?;

        assert_eq!(cfg.params.jobs, fixtures::JOBS);
        Ok(())
    }
}
