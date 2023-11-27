mod task_cfg;

use std::collections::HashMap;

use serde::Deserialize;

use crate::error::CmdResult;

pub use self::task_cfg::TaskCfg;

#[derive(Debug, Deserialize)]
pub struct Cfg {
    #[serde(rename = "task")]
    pub tasks: HashMap<String, TaskCfg>,
}

impl Cfg {
    pub fn load() -> CmdResult<Self> {
        let cfg_content = std::fs::read_to_string("cmdrc.toml")?;
        Ok(toml::from_str(&cfg_content)?)
    }
}
