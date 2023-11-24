use serde::Deserialize;

use super::common::CommonCfg;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::CommandParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParallelCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::ParallelParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SequentialCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::SequentialParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShellCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::ShellParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchCfg {
    #[serde(flatten)]
    pub common: CommonCfg,
    #[serde(flatten)]
    pub params: crate::task::modes::WatchParams,
}

#[cfg(test)]
mod tests {
    use crate::error::CmdResult;

    use super::*;

    #[test]
    fn deserialize() -> CmdResult<()> {
        let cfg: CommandCfg = toml::from_str(
            r#"
private = true
description = "test-desc"
command = "test"
args = ["test1", "test2"]
            "#,
        )?;

        assert!(cfg.common.private());
        assert_eq!(cfg.common.description(), "test-desc");
        assert_eq!(cfg.params.command, "test");
        assert_eq!(cfg.params.args, vec!["test1", "test2"]);

        let cfg: ParallelCfg = toml::from_str(
            r#"
private = false
description = "test"
tasks = ["test-task1", "test-task2"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test");
        assert_eq!(cfg.params.tasks, vec!["test-task1", "test-task2"]);

        let cfg: SequentialCfg = toml::from_str(
            r#"
private = false
description = "test-desc"
tasks = ["test"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test-desc");
        assert_eq!(cfg.params.tasks, vec!["test"]);

        let cfg: ShellCfg = toml::from_str(
            r#"
private = false
description = "test-desc"
script = """
test1
test2
"""
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test-desc");
        assert_eq!(cfg.params.script, "test1\ntest2\n");

        let cfg: WatchCfg = toml::from_str(
            r#"
private = false
description = "test2"
task = "test-task"
watch_list = ["test1", "test2"]
            "#,
        )?;

        assert!(!cfg.common.private());
        assert_eq!(cfg.common.description(), "test2");
        assert_eq!(cfg.params.task, "test-task");
        assert_eq!(cfg.params.watch_list, vec!["test1", "test2"]);
        Ok(())
    }
}
