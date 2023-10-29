pub mod task_config;

use std::collections::HashMap;

use serde::Deserialize;

pub use self::task_config::TaskConfig;

#[derive(Debug, Deserialize)]
pub struct CmdConfig {
    #[serde(rename = "task")]
    pub tasks: HashMap<String, TaskConfig>,
}
