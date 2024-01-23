use std::path::PathBuf;

use crate::error::{InternalError, JfResult};

use super::logger::{self, LogLevel, Logger};
use super::models::{
    action::{Action, Configured, Statics},
    Ctx, Opts,
};

const AUTHOR: &str = "ysuzuki19";

#[derive(clap::Parser)]
#[cfg_attr(test, derive(Default))]
#[command(
    author = AUTHOR,
    version,
    disable_version_flag = true,
    about,
    long_about = None,
    disable_help_flag = true,
)]
pub struct Args {
    #[arg(long)]
    version: bool,

    #[arg(long)]
    help: bool,

    #[arg(long)]
    validate: bool,

    #[arg(long)]
    cfg: Option<PathBuf>,

    #[arg(long, default_value = "error")]
    log_level: LogLevel,

    #[arg(long)]
    completion: Option<clap_complete::Shell>,

    #[arg(long)]
    list: bool,

    #[arg(long)]
    description: bool,

    #[command()]
    job_name: Option<String>,
}

impl Args {
    pub fn setup<LR: logger::LogWriter>(&self) -> JfResult<(Ctx<LR>, Action, Opts)> {
        let ctx = self.setup_ctx();
        let action = self.setup_action()?;
        let opts = self.setup_opts();
        Ok((ctx, action, opts))
    }

    fn setup_ctx<LR: logger::LogWriter>(&self) -> Ctx<LR> {
        Ctx {
            logger: Logger::new(self.log_level),
        }
    }

    fn setup_opts(&self) -> Opts {
        Opts {
            cfg: self.cfg.clone(),
        }
    }

    fn setup_action(&self) -> JfResult<Action> {
        if self.version {
            Ok(Statics::Version.into())
        } else if self.help {
            Ok(Statics::Help.into())
        } else if let Some(shell) = self.completion {
            Ok(Statics::Completion(shell).into())
        } else if self.list {
            Ok(Configured::List.into())
        } else if self.validate {
            Ok(Configured::Validate.into())
        } else if self.description {
            if let Some(job_name) = self.job_name.clone() {
                Ok(Configured::Description(job_name).into())
            } else {
                Err(InternalError::NeedJobNameForDescription.into())
            }
        } else if let Some(job_name) = self.job_name.clone() {
            Ok(Configured::Run(job_name).into())
        } else {
            Ok(Statics::Help.into())
        }
    }
}

#[cfg(test)]
pub mod fixtures {
    pub const APP_NAME: &str = "jf";
    pub const JOB_NAME: &str = "test-job-name";
    pub const CFG_PATH: &str = "test-cfg-path";
    pub const SIMPLE: &[&str] = &[APP_NAME, JOB_NAME];
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use clap_complete::Shell;
    use logger::MockLogWriter;

    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn parse() {
        let args = Args::parse_from(fixtures::SIMPLE);
        assert!(!args.version);
        assert!(!args.help);
        assert!(!args.validate);
        assert_eq!(args.cfg, None);
        assert!(args.log_level == LogLevel::Error);
        assert_eq!(args.completion, None);
        assert!(!args.list);
        assert!(!args.description);
        assert_eq!(args.job_name, Some(fixtures::JOB_NAME.to_string()));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup() -> JfResult<()> {
        let args = Args::default();

        let (ctx, action, opts) = args.setup::<logger::MockLogWriter>()?;
        assert!(ctx == args.setup_ctx());
        assert!(action == args.setup_action()?);
        assert!(opts == args.setup_opts());
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_ctx() {
        let args = Args::parse_from([fixtures::APP_NAME, "--log-level", "error"]);

        let ctx = args.setup_ctx::<MockLogWriter>();
        assert!(ctx.logger.level() == LogLevel::Error);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_opts() {
        let args = Args::parse_from([fixtures::APP_NAME, "--cfg", fixtures::CFG_PATH]);

        let opts = args.setup_opts();
        assert_eq!(opts.cfg, Some(PathBuf::from(fixtures::CFG_PATH)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_completion() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--completion", "bash"]);

        let action = args.setup_action()?;
        assert!(matches!(
            action,
            Action::Statics(Statics::Completion(Shell::Bash))
        ));
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_list() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--list"]);

        let action = args.setup_action()?;
        assert!(matches!(action, Action::Configured(Configured::List)));
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_validate() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--validate"]);

        let action = args.setup_action()?;
        assert!(matches!(action, Action::Configured(Configured::Validate)));
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_description() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--description", fixtures::JOB_NAME]);

        let action = args.setup_action()?;
        assert!(matches!(
            action,
            Action::Configured(Configured::Description(jn)) if jn == fixtures::JOB_NAME
        ));
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_description_without_job_name() {
        let args = Args::parse_from([fixtures::APP_NAME, "--description"]);

        let action = args.setup_action();
        assert!(action.is_err());
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_run() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, fixtures::JOB_NAME]);

        let action = args.setup_action()?;
        assert!(matches!(
            action,
            Action::Configured(Configured::Run(jn)) if jn == fixtures::JOB_NAME
        ));
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_help() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--help"]);

        let action = args.setup_action()?;
        assert!(matches!(action, Action::Statics(Statics::Help)));
        Ok(())
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn setup_action_version() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--version"]);

        let action = args.setup_action()?;
        assert!(matches!(action, Action::Statics(Statics::Version)));
        Ok(())
    }
}
