#[derive(Debug, Clone, serde::Deserialize)]
pub struct SequentialCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
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
jobs = ["test"]
"#,
        )?;

        assert_eq!(cfg.params.jobs, vec!["test"]);
        Ok(())
    }
}
