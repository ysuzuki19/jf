use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CommonCfg {
    #[serde(default)]
    private: bool,
    #[serde(default)]
    description: String,
}

impl CommonCfg {
    pub fn private(&self) -> bool {
        self.private
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}
