#[derive(Debug, Clone, serde::Deserialize)]
pub struct ParallelCfg {
    #[serde(flatten)]
    pub common: super::super::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::ParallelParams,
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: ParallelCfg = toml::from_str(
            r#"
jobs = ["test-job1", "test-job2"]
"#,
        )?;

        assert_eq!(cfg.params.jobs, vec!["test-job1", "test-job2"]);
        Ok(())
    }
}
