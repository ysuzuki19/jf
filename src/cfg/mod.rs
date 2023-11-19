pub mod task_cfg;

use std::collections::HashMap;

use serde::Deserialize;

pub use self::task_cfg::TaskCfg;

#[derive(Debug, Deserialize)]
pub struct CmdCfg {
    #[serde(rename = "task")]
    pub tasks: HashMap<String, TaskCfg>,
}
