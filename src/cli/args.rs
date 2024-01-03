use std::path::PathBuf;

use clap::Parser;

use crate::error::{InternalError, JfResult};

use super::models::{
    action::{Action, Configured, Statics},
    Ctx, LogLevel, Logger, Opts,
};

const AUTHOR: &str = "ysuzuki19";

#[derive(Parser, Debug, Clone)]
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
    pub version: bool,

    #[arg(long)]
    pub help: bool,

    #[arg(long)]
    pub validate: bool,

    #[arg(long)]
    pub cfg: Option<PathBuf>,

    #[arg(long, default_value = "error")]
    pub log_level: LogLevel,

    #[arg(long)]
    pub completion: Option<clap_complete::Shell>,

    #[arg(long)]
    pub list: bool,

    #[arg(long)]
    pub description: bool,

    #[command()]
    pub job_name: Option<String>,
}

impl Args {
    pub fn setup(&self) -> JfResult<(Ctx, Action, Opts)> {
        let ctx = self.setup_ctx();
        let action = self.setup_action()?;
        let opts = self.setup_opts();
        Ok((ctx, action, opts))
    }

    fn setup_ctx(&self) -> Ctx {
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
mod fixtures {
    const APP_NAME: &str = "jf";
    pub const SIMPLE:&[&str] = &[APP_NAME, "test"];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let args = Args::parse_from(fixtures::SIMPLE);
        assert!(!args.version);
        assert!(!args.help);
        assert!(!args.validate);
        assert_eq!(args.cfg, None);
        assert_eq!(args.log_level, LogLevel::Error);
        assert_eq!(args.completion, None);
        assert!(!args.list);
        assert!(!args.description);
        assert_eq!(args.job_name, Some("test".to_string()));
    }

    #[test]
    fn setup() -> JfResult<()> {
        let args = Args::default();

        let (ctx, action, opts) = args.setup()?;
        assert_eq!(ctx, args.setup_ctx());
        assert_eq!(action, args.setup_action()?);
        assert_eq!(opts, args.setup_opts());
        Ok(())
    }

    #[test]
    fn setup_ctx() {
        let args = Args {
            log_level: LogLevel::Error,
            ..Default::default()
        };

        let ctx = args.setup_ctx();
        assert_eq!(ctx.logger.level(), LogLevel::Error);
    }

    #[test]
    fn setup_opts() {
        let cfg = PathBuf::from("test");
        let args = Args {
            cfg: Some(cfg.clone()),
            ..Default::default()
        };

        let opts = args.setup_opts();
        assert_eq!(opts.cfg, Some(cfg));
    }

    #[test]
    fn setup_action_completion() -> JfResult<()> {
        let args = Args {
            completion: Some(clap_complete::Shell::Bash),
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(
            action,
            Action::Statics(Statics::Completion(clap_complete::Shell::Bash))
        );
        Ok(())
    }

    #[test]
    fn setup_action_list() -> JfResult<()> {
        let args = Args {
            list: true,
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(action, Action::Configured(Configured::List));
        Ok(())
    }

    #[test]
    fn setup_action_validate() -> JfResult<()> {
        let args = Args {
            validate: true,
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(action, Action::Configured(Configured::Validate));
        Ok(())
    }

    #[test]
    fn setup_action_description() -> JfResult<()> {
        let job_name = "test".to_string();
        let args = Args {
            description: true,
            job_name: Some(job_name.clone()),
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(
            action,
            Action::Configured(Configured::Description(jn)) if jn == job_name
        );
        Ok(())
    }

    #[test]
    fn setup_action_description_without_job_name() {
        let args = Args {
            description: true,
            ..Default::default()
        };

        let action = args.setup_action();
        assert!(action.is_err());
    }

    #[test]
    fn setup_action_run() -> JfResult<()> {
        let job_name = "test".to_string();
        let args = Args {
            job_name: Some(job_name.clone()),
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(
            action,
            Action::Configured(Configured::Run(jn)) if jn == job_name
        );
        Ok(())
    }

    #[test]
    fn setup_action_help() -> JfResult<()> {
        let args = Args {
            help: true,
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(action, Action::Statics(Statics::Help));
        Ok(())
    }

    #[test]
    fn setup_action_version() -> JfResult<()> {
        let args = Args {
            version: true,
            ..Default::default()
        };

        let action = args.setup_action()?;
        matches!(action, Action::Statics(Statics::Version));
        Ok(())
    }
}
