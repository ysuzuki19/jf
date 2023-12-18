pub mod job_cfg;

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::JfResult;

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
        let content = std::fs::read_to_string(DEFAULT_CFG)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn load_with_path(path: &str) -> JfResult<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
