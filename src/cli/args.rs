// SPDX-License-Identifier: MPL-2.0
use std::path::PathBuf;

use crate::{
    ctx::Ctx,
    logging::{LogLevel, Logger},
    util::error::{IntoJfError, JfResult},
};

use super::models::{
    action::{Action, Configured, Statics},
    Opts,
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
    init: Option<super::models::action::init::Mode>,

    #[arg(long)]
    validate: bool,

    #[arg(long)]
    cfg: Option<PathBuf>,

    #[arg(long, default_value = "info")]
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
    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    pub fn setup(&self, logger: Logger) -> JfResult<(Ctx, Action, Opts)> {
        let ctx = self.setup_ctx(logger);
        let action = self.setup_action()?;
        let opts = self.setup_opts();
        Ok((ctx, action, opts))
    }

    fn setup_ctx(&self, logger: Logger) -> Ctx {
        Ctx::new(logger)
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
        } else if let Some(init_mode) = self.init {
            Ok(Statics::Init(init_mode).into())
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
                Err("Please input <JOB_NAME> to use --description".into_jf_error())
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

    use crate::{logging::LoggingMock, util::testutil::async_test};

    use super::*;

    #[test]
    #[coverage(off)]
    fn parse() {
        let args = Args::parse_from(fixtures::SIMPLE);
        assert!(!args.version);
        assert!(!args.help);
        assert!(!args.validate);
        assert_eq!(args.cfg, None);
        assert_eq!(args.log_level, LogLevel::Info);
        assert_eq!(args.log_level(), LogLevel::Info);
        assert_eq!(args.completion, None);
        assert!(!args.list);
        assert!(!args.description);
        assert_eq!(args.job_name, Some(fixtures::JOB_NAME.to_string()));
    }

    #[test]
    #[coverage(off)]
    fn setup() -> JfResult<()> {
        async_test(
            #[coverage(off)]
            async move {
                let args = Args::default();

                let logging_mock = LoggingMock::new().await;
                let (ctx, action, opts) = args.setup(logging_mock.logger.clone())?;
                assert_eq!(ctx, args.setup_ctx(logging_mock.logger));
                assert_eq!(action, args.setup_action()?);
                assert_eq!(opts, args.setup_opts());
                Ok(())
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn setup_ctx() {
        async_test(
            #[coverage(off)]
            async move {
                let args = Args::parse_from([fixtures::APP_NAME, "--log-level", "error"]);

                let logging_mock = LoggingMock::new().await;
                let ctx = args.setup_ctx(logging_mock.logger.clone().update(LogLevel::Error));
                assert_eq!(ctx.logger().level(), LogLevel::Error);
            },
        )
    }

    #[test]
    #[coverage(off)]
    fn setup_opts() {
        let args = Args::parse_from([fixtures::APP_NAME, "--cfg", fixtures::CFG_PATH]);

        let opts = args.setup_opts();
        assert_eq!(opts.cfg, Some(PathBuf::from(fixtures::CFG_PATH)));
    }

    #[test]
    #[coverage(off)]
    fn setup_action_completion() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--completion", "bash"]);

        let action = args.setup_action()?;
        assert_eq!(action, Action::Statics(Statics::Completion(Shell::Bash)));
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn setup_action_list() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--list"]);

        let action = args.setup_action()?;
        assert_eq!(action, Action::Configured(Configured::List));
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn setup_action_validate() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--validate"]);

        let action = args.setup_action()?;
        assert_eq!(action, Action::Configured(Configured::Validate));
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn setup_action_description() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--description", fixtures::JOB_NAME]);

        let action = args.setup_action()?;
        assert_eq!(
            action,
            Action::Configured(Configured::Description(fixtures::JOB_NAME.to_owned()))
        );
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn setup_action_description_without_job_name() {
        let args = Args::parse_from([fixtures::APP_NAME, "--description"]);

        let action = args.setup_action();
        assert!(action.is_err());
    }

    #[test]
    #[coverage(off)]
    fn setup_action_run() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, fixtures::JOB_NAME]);

        let action = args.setup_action()?;
        assert_eq!(
            action,
            Action::Configured(Configured::Run(fixtures::JOB_NAME.to_owned()))
        );
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn setup_action_help() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--help"]);

        let action = args.setup_action()?;
        assert_eq!(action, Action::Statics(Statics::Help));
        Ok(())
    }

    #[test]
    #[coverage(off)]
    fn setup_action_version() -> JfResult<()> {
        let args = Args::parse_from([fixtures::APP_NAME, "--version"]);

        let action = args.setup_action()?;
        assert_eq!(action, Action::Statics(Statics::Version));
        Ok(())
    }
}
