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

#[cfg(test)]
mod tests {
    use crate::error::CmdResult;

    use super::*;

    #[test]
    fn deserialize() -> CmdResult<()> {
        let cfg: CommonCfg = toml::from_str(
            r#"
            private = true
            description = "test"
            "#,
        )?;

        assert!(cfg.private);
        assert_eq!(cfg.description, "test");

        let cfg: CommonCfg = toml::from_str(
            r#"
            private = false
            description = "test2"
            "#,
        )?;

        assert!(!cfg.private);
        assert_eq!(cfg.description, "test2");

        let cfg: CommonCfg = toml::from_str(r#""#)?;

        assert!(!cfg.private);
        assert_eq!(cfg.description, "");
        Ok(())
    }
}
