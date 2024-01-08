use serde::Deserialize;

use super::Visibility;

#[derive(Deserialize)]
pub struct CommonCfg {
    #[serde(default)]
    visibility: Visibility,
    #[serde(default)]
    description: String,
}

impl CommonCfg {
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
impl CommonCfg {
    #[coverage(off)]
    pub fn new(visibility: Visibility, description: String) -> Self {
        Self {
            visibility,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    #[coverage(off)]
    fn deserialize_default() -> JfResult<()> {
        let cfg: CommonCfg = toml::from_str("")?;

        assert!(cfg.visibility().is_public());
        assert_eq!(cfg.description, "");
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn deserialize_private() -> JfResult<()> {
        let cfg: CommonCfg = toml::from_str(
            r#"
visibility = "private"
description = "test"
"#,
        )?;

        assert!(cfg.visibility().is_private());
        assert_eq!(cfg.description, "test");
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn deserialize_public() -> JfResult<()> {
        let cfg: CommonCfg = toml::from_str(
            r#"
visibility = "public"
description = "test2"
"#,
        )?;

        assert!(cfg.visibility().is_public());
        assert_eq!(cfg.description, "test2");
        Ok(())
    }
}
