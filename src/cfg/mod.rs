// SPDX-License-Identifier: MPL-2.0
pub mod cfg_path_gen;
pub mod job_cfg;

use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::util::error::JfResult;

#[derive(Deserialize)]
pub struct Cfg {
    #[serde(rename = "job")]
    pub jobs: HashMap<String, job_cfg::JobCfg>,
}

impl Cfg {
    pub fn load(cfg: Option<PathBuf>) -> JfResult<Self> {
        let file_path = cfg_path_gen::CfgPathGen::new(cfg).gen();
        match std::fs::read_to_string(file_path) {
            Ok(c) => Ok(toml::from_str(&c)?),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(Self {
                    jobs: HashMap::new(),
                }),
                _ => Err(crate::util::error::JfError::IoError(e)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[coverage(off)]
    fn load() -> JfResult<()> {
        Cfg::load(Some(cfg_path_gen::tests::fixtures_dir()))?;
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn load_unexist() -> JfResult<()> {
        let cfg = Cfg::load(Some(cfg_path_gen::tests::unexist_dir()))?;
        assert_eq!(cfg.jobs.len(), 0);
        Ok(())
    }
}
