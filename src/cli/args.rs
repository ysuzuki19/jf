use std::path::PathBuf;

use clap::Parser;

use crate::error::{InternalError, JfResult};

use super::{
    action::{Action, Configured, Static},
    containers::{Ctx, Opts},
    LogLevel,
};

const AUTHOR: &str = "ysuzuki19";

#[derive(Parser, Debug, Clone)]
#[command(
    author = AUTHOR,
    version,
    about,
    long_about = None,
    disable_help_flag = true,
)]
pub struct Args {
    #[arg(long)]
    pub(super) help: bool,

    #[arg(long)]
    pub(super) validate: bool,

    #[arg(short, long)]
    pub(super) cfg: Option<PathBuf>,

    #[arg(long, default_value = "error")]
    pub log_level: LogLevel,

    #[arg(long)]
    pub(super) completion: Option<clap_complete::Shell>,

    #[arg(long)]
    pub(super) list: bool,

    #[arg(long)]
    pub(super) description: bool,

    #[command()]
    pub(super) job_name: Option<String>,
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
            log_level: self.log_level,
        }
    }

    fn setup_opts(&self) -> Opts {
        Opts {
            cfg: self.cfg.clone(),
        }
    }

    fn setup_action(&self) -> JfResult<Action> {
        if let Some(shell) = self.completion {
            Ok(Static::Completion(shell).into())
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
            Ok(Static::Help.into())
        }
    }
}
