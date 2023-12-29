use std::path::PathBuf;

use clap::Parser;

use crate::error::{JfError, JfResult};

use super::{
    behavior::{CliBehavior, Configured, Static},
    containers::{Context, Options},
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
    pub fn setup(&self) -> JfResult<(Context, CliBehavior, Options)> {
        let ctx = self.setup_context();
        let behavior = self.setup_behavior()?;
        let options = Options {
            cfg: self.cfg.clone(),
        };
        Ok((ctx, behavior, options))
    }

    fn setup_context(&self) -> Context {
        Context {
            log_level: self.log_level.clone(),
        }
    }

    fn setup_behavior(&self) -> JfResult<CliBehavior> {
        if let Some(shell) = self.completion {
            Ok(CliBehavior::Static(Static::Completion { shell }))
        } else if self.list {
            Ok(CliBehavior::Configured(Configured::List))
        } else if self.validate {
            Ok(CliBehavior::Configured(Configured::Validate))
        } else if self.description {
            if let Some(job_name) = self.job_name.clone() {
                Ok(CliBehavior::Configured(Configured::Description {
                    job_name,
                }))
            } else {
                Err(JfError::Custom(
                    "Please input <JOB_NAME> to --description".to_string(),
                ))
            }
        } else if let Some(job_name) = self.job_name.clone() {
            Ok(CliBehavior::Configured(Configured::Run { job_name }))
        } else {
            Ok(CliBehavior::Static(Static::Help))
        }
    }
}
