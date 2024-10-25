// SPDX-License-Identifier: MPL-2.0
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
    use crate::util::{error::JfResult, testutil::Fixture};

    use super::*;

    impl Fixture for MockCfg {
        #[coverage(off)]
        fn fixture() -> Self {
            Self {
                common: Fixture::fixture(),
                params: Fixture::fixture(),
            }
        }
    }

    #[test]
    #[coverage(off)]
    fn deserialize() -> JfResult<()> {
        let cfg: MockCfg = toml::from_str(fixtures::SIMPLE)?;

        assert_eq!(cfg.params.each_sleep_time, 1);
        assert_eq!(cfg.params.sleep_count, 3);

        Ok(())
    }
}
