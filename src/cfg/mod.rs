mod cfg_path_gen;
pub mod job_cfg;

use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::error::JfResult;

#[derive(Deserialize)]
pub struct Cfg {
    #[serde(rename = "job")]
    pub jobs: HashMap<String, job_cfg::JobCfg>,
}

impl Cfg {
    pub fn load(cfg: Option<PathBuf>) -> JfResult<Self> {
        let file_path = cfg_path_gen::CfgPathGen::new(cfg).gen();
        let content = std::fs::read_to_string(file_path)?;
        Ok(toml::from_str(&content)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::JfError;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn load() -> JfResult<()> {
        Cfg::load(Some(cfg_path_gen::tests::fixtures_dir()))?;
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn load_unexist() -> JfResult<()> {
        let must_fail = Cfg::load(Some(cfg_path_gen::tests::unexist_dir()));
        assert!(must_fail.is_err());
        assert!(matches!(must_fail.err().unwrap(), JfError::IoError(_)));
        Ok(())
    }
}
