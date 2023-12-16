mod task_cfg;

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::JfResult;

pub use self::task_cfg::TaskCfg;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    #[serde(rename = "task")]
    pub tasks: HashMap<String, TaskCfg>,
}

impl Cfg {
    pub fn load() -> JfResult<Self> {
        let cfg_content = std::fs::read_to_string("jfrc.toml")?;
        Ok(toml::from_str(&cfg_content)?)
    }
}
