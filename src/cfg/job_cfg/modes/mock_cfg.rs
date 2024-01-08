#[derive(serde::Deserialize)]
pub struct MockCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::MockParams,
}

#[cfg(test)]
pub mod fixtures {
    pub const SIMPLE: &str = r#"
description = "test-desc"
each_sleep_time = 1
sleep_count = 3"#;
}

#[cfg(test)]
mod tests {
    use crate::{error::JfResult, testutil::Fixture};

    use super::*;

    impl Fixture for MockCfg {
        #[cfg_attr(coverage, coverage(off))]
        fn gen() -> Self {
            Self {
                common: Fixture::gen(),
                params: Fixture::gen(),
            }
        }
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn deserialize() -> JfResult<()> {
        let cfg: MockCfg = toml::from_str(fixtures::SIMPLE)?;

        assert_eq!(cfg.params.each_sleep_time, 1);
        assert_eq!(cfg.params.sleep_count, 3);

        Ok(())
    }
}
