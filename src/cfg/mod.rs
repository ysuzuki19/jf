pub mod job_cfg;

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::JfResult;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    #[serde(rename = "job")]
    pub jobs: HashMap<String, job_cfg::JobCfg>,
}

impl Cfg {
    pub fn load() -> JfResult<Self> {
        let cfg_content = std::fs::read_to_string("jfrc.toml")?;
        Ok(toml::from_str(&cfg_content)?)
    }
}
