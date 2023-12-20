use clap::Parser;

use crate::error::JfError;

#[derive(Parser, Debug, Clone)]
#[command(
    author = "ysuzuki19",
    version,
    about,
    long_about = None,
    disable_help_flag = true,
)]
pub struct Args {
    #[arg(long)]
    verbose: bool,

    #[arg(short, long)]
    pub cfg: Option<String>,

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
    pub fn list(&self) -> bool {
        self.list
    }
}

pub enum Static {
    Completion { shell: clap_complete::Shell },
    Help,
}

pub enum Configured {
    List,
    Description { job_name: String },
    Run { job_name: String },
}

pub enum CliBehavior {
    Static(Static),
    Configured(Configured),
}

impl TryFrom<Args> for CliBehavior {
    type Error = JfError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        if let Some(shell) = args.completion {
            Ok(Self::Static(Static::Completion { shell }))
        } else if args.list {
            Ok(Self::Configured(Configured::List))
        } else if args.description {
            if let Some(job_name) = args.job_name {
                Ok(Self::Configured(Configured::Description { job_name }))
            } else {
                Err(JfError::Custom(
                    "Please input <JOB_NAME> to --description".to_string(),
                ))
            }
        } else if let Some(job_name) = args.job_name {
            Ok(Self::Configured(Configured::Run { job_name }))
        } else {
            Ok(Self::Static(Static::Help))
        }
    }
}
