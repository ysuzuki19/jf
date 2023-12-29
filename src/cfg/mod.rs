mod cfg_path_gen;
pub mod job_cfg;

use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::error::JfResult;

#[derive(Debug, Deserialize)]
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
