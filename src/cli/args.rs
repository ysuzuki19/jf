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

// Sorted by priority
pub enum CliBehavior {
    Completion { shell: clap_complete::Shell },
    List,
    Description { job_name: String },
    Run { job_name: String },
    Help,
}

impl TryFrom<Args> for CliBehavior {
    type Error = JfError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        if let Some(shell) = args.completion {
            Ok(Self::Completion { shell })
        } else if args.list {
            Ok(Self::List)
        } else if args.description {
            if let Some(job_name) = args.job_name {
                return Ok(Self::Description { job_name });
            } else {
                return Err(JfError::Custom(
                    "Please input <JOB_NAME> to --description".to_string(),
                ));
            }
        } else if let Some(job_name) = args.job_name {
            Ok(Self::Run { job_name })
        } else {
            Ok(Self::Help)
        }
    }
}
