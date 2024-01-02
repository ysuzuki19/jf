#[derive(Debug, Clone, serde::Deserialize)]
pub struct MockCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::MockParams,
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: MockCfg = toml::from_str(
            r#"
each_sleep_time = 1
sleep_count = 3
"#,
        )?;

        assert_eq!(cfg.params.each_sleep_time, 1);
        assert_eq!(cfg.params.sleep_count, 3);

        Ok(())
    }
}
