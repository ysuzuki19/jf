pub mod job_cfg;

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::{JfError, JfResult};

#[derive(Debug, Deserialize)]
pub struct Cfg {
    #[serde(rename = "job")]
    pub jobs: HashMap<String, job_cfg::JobCfg>,
}

const DEFAULT_CFG: &str = "jf.toml";

impl Cfg {
    pub fn load(cfg_path: Option<String>) -> JfResult<Self> {
        match cfg_path {
            Some(path) => Self::load_with_path(&path),
            None => Self::load_default(),
        }
    }

    pub fn load_default() -> JfResult<Self> {
        let cfg_content = std::fs::read_to_string(DEFAULT_CFG)
            .map_err(|_| JfError::Custom(DEFAULT_CFG.to_string()))?;
        Ok(toml::from_str(&cfg_content)?)
    }

    pub fn load_with_path(path: &str) -> JfResult<Self> {
        let cfg_content =
            std::fs::read_to_string(path).map_err(|_| JfError::Custom(path.to_string()))?;
        Ok(toml::from_str(&cfg_content)?)
    }
}
