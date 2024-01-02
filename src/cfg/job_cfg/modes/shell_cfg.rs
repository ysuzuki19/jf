#[derive(Debug, Clone, serde::Deserialize)]
pub struct ShellCfg {
    #[serde(flatten)]
    pub common: crate::cfg::job_cfg::common::CommonCfg,
    #[serde(flatten)]
    pub params: crate::job::modes::ShellParams,
}

#[cfg(test)]
mod tests {
    use crate::error::JfResult;

    use super::*;

    #[test]
    fn deserialize() -> JfResult<()> {
        let cfg: ShellCfg = toml::from_str(
            r#"
script = """
test1
test2
"""
"#,
        )?;

        assert_eq!(cfg.params.script, "test1\ntest2\n");
        Ok(())
    }
}
